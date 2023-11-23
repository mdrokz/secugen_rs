use std::path::PathBuf;

use std::env;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").expect("No out dir found"));

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let bindings = bindgen::Builder::default()
        .clang_args(&["-x", "c++", "-std=c++11"])
        .header("./include/sgfplib.h")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(&out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rustc-link-lib=sgfplib");

    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-search=native={}/libs/linux", manifest_dir);
        println!("cargo:rustc-link-lib=jpeg");
        println!("cargo:rustc-link-lib=stdc++");
    } else if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-search=native={}/libs/windows", manifest_dir);
        println!("cargo:rustc-link-lib=sgfplib");
    }
}
