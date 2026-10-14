use mlua::prelude::*;
use std::process::ExitCode;

mod ops;

const ENTRYPOINT: &[u8] = include_bytes!("../runtime/init.luau");

fn run() -> LuaResult<()> {
    let luau = Lua::new();
    luau.set_memory_category("graphite")?;

    luau.load_std_libs(LuaStdLib::ALL)?;
    luau.globals().set("require", LuaNil)?;
    luau.gc_collect()?;
    luau.set_jit_options(LuauJitOptions::new().inliner(true));
    luau.enable_jit(true);

    let ops = ops::create(&luau)?;
    luau.load(ENTRYPOINT)
        .set_name("@@runtime/init.luau")
        .call(ops)
}

fn main() -> ExitCode {
    match run() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{}", e.to_string());
            ExitCode::FAILURE
        }
    }
}
