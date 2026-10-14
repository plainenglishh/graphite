use std::env;
use std::fs;
use std::path::Path;

use mlua::chunk::Compiler;

fn main() {
    println!("cargo:rerun-if-changed=runtime");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("std.rs");

    let mut vfs_code = String::new();
    vfs_code.push_str("pub static STD: &[(&str, &[u8])] = &[\n");

    let base_dir = Path::new("runtime");
    if base_dir.exists() {
        visit_dirs(base_dir, base_dir, &mut vfs_code);
    } else {
        panic!("runtime doesnt exist");
    }

    vfs_code.push_str("];\n");

    fs::write(&dest_path, vfs_code).unwrap();
}

fn visit_dirs(dir: &Path, base_dir: &Path, vfs_code: &mut String) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();

            if path.is_dir() {
                visit_dirs(&path, base_dir, vfs_code);
            } else if path.is_file() && path.extension().map_or(false, |ext| ext == "luau") {
                let relative_path = path.strip_prefix(base_dir).unwrap();
                let file_stem = relative_path.file_stem().unwrap().to_str().unwrap();

                let module_key = if let Some(parent) = relative_path.parent() {
                    if parent.as_os_str().is_empty() {
                        file_stem.to_string()
                    } else {
                        let parent_str = parent.to_str().unwrap().replace("\\", "/");
                        format!("{}/{}", parent_str, file_stem)
                    }
                } else {
                    file_stem.to_string()
                };

                let bytecode = Compiler::new()
                    .set_optimization_level(2)
                    .set_debug_level(2)
                    .set_coverage_level(0)
                    .set_type_info_level(0)
                    .compile(fs::read(&path).unwrap())
                    .unwrap();

                vfs_code.push_str(&format!("    (\"{}\", &{:?}),\n", module_key, bytecode));
            }
        }
    }
}
