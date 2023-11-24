use std::path::PathBuf;

use std::{env, fs};

fn main() {
    let mut out_path = PathBuf::from(env::var("OUT_DIR").expect("No out dir found"));

    let bindings = bindgen::Builder::default()
        .clang_args(&["-x", "c++", "-std=c++11"])
        .header("./include/sgfplib.h")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(&out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    out_path.pop();
    out_path.pop();
    out_path.pop();

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    if cfg!(target_os = "windows") {
        let source_path = format!("{}/libs/windows", manifest_dir);
        for entry in fs::read_dir(source_path).expect("Failed to read source directory") {
            let entry = entry.expect("Failed to read directory entry");
            let file_path = entry.path();

            if file_path.is_file() {
                // Construct the target file path.
                let target_file_path = out_path.join(file_path.file_name().unwrap());
                fs::copy(file_path, target_file_path).expect("Failed to copy file");
            }
        }
    }

    println!("cargo:rustc-link-lib=sgfplib");

    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-search=native={}/libs/linux", manifest_dir);
        println!("cargo:rustc-link-lib=jpeg");
        println!("cargo:rustc-link-lib=stdc++");
    } else if cfg!(target_os = "windows") {
        println!(
            "cargo:rustc-link-search=native={}/libs/windows",
            manifest_dir
        );
        println!("cargo:rustc-link-lib=sgfplib");
    }
}
