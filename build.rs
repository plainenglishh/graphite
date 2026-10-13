// build.rs
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

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

            /*let output = Command::new("luau-compile")
            .arg("--binary")
            .arg("-O2")
            .arg("-g1")
            .arg(&path)
            .output()
            .unwrap();*/

            //let bytecode = output.stdout;
            vfs_code.push_str(&format!(
                "    (\"{}\", &{:?}),\n",
                module_key,
                fs::read(path).unwrap().to_vec()
            ));
        }
    }

    vfs_code.push_str("];\n");

    fs::write(&dest_path, vfs_code).unwrap();
}
