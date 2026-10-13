// build.rs
use std::env;
use std::fs;
use std::path::Path;

use mlua::chunk::Compiler;

fn main() {
    println!("cargo:rerun-if-changed=std");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("std.rs");

    let mut vfs_code = String::new();
    vfs_code.push_str("pub static STD: &[(&str, &[u8])] = &[\n");

    let entries = fs::read_dir("rt/std").expect("Failed to read std directory");

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() && path.extension().map_or(false, |ext| ext == "luau") {
            let file_name = path.file_stem().unwrap().to_str().unwrap();
            let module_key = format!("{}", file_name);

            let bytecode = Compiler::new()
                .set_optimization_level(2)
                .set_debug_level(2)
                .set_coverage_level(0)
                .set_type_info_level(0)
                .compile(fs::read(path).unwrap())
                .unwrap();

            vfs_code.push_str(&format!("    (\"{}\", &{:?}),\n", module_key, bytecode));
        }
    }

    vfs_code.push_str("];\n");

    fs::write(&dest_path, vfs_code).unwrap();
}
