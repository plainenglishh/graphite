use std::{env, ffi::OsString, fs, process};

use mlua::{ffi, prelude::*};

include!(concat!(env!("OUT_DIR"), "/std.rs"));

fn find_std_bytecode(module_name: &str) -> Option<&'static [u8]> {
    STD.iter()
        .find(|&&(key, _)| key == module_name)
        .map(|&(_, bytes)| bytes)
}

fn op_std_module(luau: &Lua, path: LuaString) -> LuaResult<LuaValue> {
    let bytes = find_std_bytecode(&path.to_string_lossy());
    match bytes {
        Some(bytes) => Ok(LuaValue::Buffer(luau.create_buffer(bytes)?)),
        None => Ok(LuaValue::Nil),
    }
}

fn op_set_global(luau: &Lua, (key, value): (LuaString, LuaValue)) -> LuaResult<()> {
    luau.globals().set(key, value)?;
    Ok(())
}

fn op_sandbox(luau: &Lua, _: ()) -> LuaResult<()> {
    luau.sandbox(true)?;
    Ok(())
}

fn op_registry(luau: &Lua, _: ()) -> LuaResult<LuaTable> {
    unsafe {
        luau.exec_raw((), |state| {
            ffi::lua_pushvalue(state, ffi::LUA_REGISTRYINDEX);
        })
    }
}

fn op_cwd(luau: &Lua, _: ()) -> LuaResult<LuaString> {
    Ok(luau.create_string(
        env::current_dir()
            .into_lua_err()?
            .as_os_str()
            .as_encoded_bytes(),
    )?)
}

fn op_args(luau: &Lua, _: ()) -> LuaResult<LuaTable> {
    let args: Vec<OsString> = env::args_os().collect();
    let args_t = luau.create_table_with_capacity(args.len(), 0)?;

    for (index, arg) in args.iter().skip(1).enumerate() {
        let lua_index = (index + 1) as i64;
        args_t.raw_set(lua_index, luau.create_string(arg.as_encoded_bytes())?)?;
    }

    Ok(args_t)
}

fn op_exit(_: &Lua, exit_code: i32) -> LuaResult<()> {
    process::exit(exit_code);
}

fn op_compile(luau: &Lua, source: LuaBuffer) -> LuaResult<LuaBuffer> {
    let bc = LuaCompiler::new().compile(source.to_vec()).into_lua_err()?;
    luau.create_buffer(bc)
}

fn op_load(luau: &Lua, (bc, debug_name): (LuaBuffer, LuaString)) -> LuaResult<LuaFunction> {
    luau.load(bc.to_vec())
        .set_name(debug_name.to_string_lossy())
        .into_function()
}

fn op_read_file(luau: &Lua, path: LuaString) -> LuaResult<LuaBuffer> {
    let path_bytes = path.as_bytes();
    let path_str = std::str::from_utf8(&path_bytes)
        .map_err(|e| mlua::Error::runtime(format!("invalid path string {}", e)))?;

    let file_bytes = fs::read(path_str)
        .map_err(|e| mlua::Error::runtime(format!("failed to read file '{}' - {}", path_str, e)))?;

    luau.create_buffer(file_bytes)
}
pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    let ops = luau.create_table()?;

    ops.set("std_module", luau.create_function(op_std_module)?)?;
    ops.set("set_global", luau.create_function(op_set_global)?)?;
    ops.set("sandbox", luau.create_function(op_sandbox)?)?;
    ops.set("registry", luau.create_function(op_registry)?)?;

    ops.set("cwd", luau.create_function(op_cwd)?)?;
    ops.set("args", luau.create_function(op_args)?)?;
    ops.set("exit", luau.create_function(op_exit)?)?;

    ops.set("compile", luau.create_function(op_compile)?)?;
    ops.set("load", luau.create_function(op_load)?)?;

    ops.set("read_file", luau.create_function(op_read_file)?)?;

    Ok(ops)
}
