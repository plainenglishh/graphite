const RT: &[u8] = include_bytes!("../rt/init.luau");

use mlua::prelude::*;
mod ops;

fn run() -> LuaResult<()> {
    let luau = Lua::new();

    luau.load_std_libs(LuaStdLib::ALL)?;
    luau.globals().set("require", LuaNil)?;

    let ops = ops::create(&luau)?;
    luau.load(RT).set_name("=graphite").call(ops)
}

fn main() {
    match run() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e.to_string())
        }
    }
}
