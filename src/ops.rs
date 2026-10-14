use mlua::{ffi, prelude::*};
use std::{env, ffi::OsString, fs, process};

include!(concat!(env!("OUT_DIR"), "/std.rs"));

fn find_std_bytecode(module_name: &str) -> Option<&'static [u8]> {
    STD.iter()
        .find(|&&(key, _)| key == module_name)
        .map(|&(_, bytes)| bytes)
}

fn op_graphite_module(luau: &Lua, path: LuaString) -> LuaResult<LuaValue> {
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

fn op_set_memory_category(luau: &Lua, name: LuaString) -> LuaResult<()> {
    luau.set_memory_category(&name.to_string_lossy())?;
    Ok(())
}

fn op_set_memory_limit(luau: &Lua, name: LuaNumber) -> LuaResult<()> {
    luau.set_memory_limit(usize::from_lua(LuaValue::Number(name), luau)?)?;
    Ok(())
}

fn op_heap_dump(luau: &Lua, (): ()) -> LuaResult<LuaTable> {
    let dump = luau.heap_dump()?;

    let categories = luau.create_table()?;
    for (category, bytes) in dump.size_by_category() {
        categories.set(category, bytes as f64)?;
    }

    let types = luau.create_table()?;
    for (typename, (count, size)) in dump.size_by_type(None) {
        let elem = luau.create_table()?;
        elem.set("count", count as f64)?;
        elem.set("size", size as f64)?;
        types.set(typename, elem)?;
    }

    let userdata = luau.create_table()?;
    for (typename, (count, size)) in dump.size_by_userdata(None) {
        let elem = luau.create_table()?;
        elem.set("count", count as f64)?;
        elem.set("size", size as f64)?;
        userdata.set(typename, elem)?;
    }

    let dump_t = luau.create_table()?;
    dump_t.set("total_size", dump.size())?;
    dump_t.set("categories", categories)?;
    dump_t.set("types", types)?;
    dump_t.set("userdata", userdata)?;

    Ok(dump_t)
}

fn op_chunk_name(luau: &Lua, level: LuaNumber) -> LuaResult<Option<LuaString>> {
    let source = luau.inspect_stack(level as _, |dbg| {
        dbg.source().source.map(|v| v.to_string()).unwrap()
    });

    Ok(source.map(|v| luau.create_string(v).unwrap()))
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

    ops.set("graphite_module", luau.create_function(op_graphite_module)?)?;
    ops.set("set_global", luau.create_function(op_set_global)?)?;
    ops.set("sandbox", luau.create_function(op_sandbox)?)?;
    ops.set("registry", luau.create_function(op_registry)?)?;
    ops.set(
        "set_memory_category",
        luau.create_function(op_set_memory_category)?,
    )?;
    ops.set(
        "set_memory_limit",
        luau.create_function(op_set_memory_limit)?,
    )?;
    ops.set("heap_dump", luau.create_function(op_heap_dump)?)?;
    ops.set("chunk_name", luau.create_function(op_chunk_name)?)?;

    ops.set("cwd", luau.create_function(op_cwd)?)?;
    ops.set("args", luau.create_function(op_args)?)?;
    ops.set("exit", luau.create_function(op_exit)?)?;

    ops.set("compile", luau.create_function(op_compile)?)?;
    ops.set("load", luau.create_function(op_load)?)?;

    ops.set("read_file", luau.create_function(op_read_file)?)?;

    Ok(ops)
}
