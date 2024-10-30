use std::{env, path::PathBuf, process::Command};
use bindgen;

fn main() {

    let current_dir = env::current_dir().unwrap();
    let bindings = bindgen::Builder::default()
        .header("path/to/header")
        .allowlist_function("ksw2_*")
        .generate()
        .expect("generate binding error");

    bindings
        .write_to_file(current_dir.join("src").join("bindings.rs"))
        .expect("binding write to file error");

    // OUT_DIR is the build output dir
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    Command::new("make")
        .current_dir("/MakefileDir")
        .output()
        .expect("build error");
    // Command::new("cp")
    //     .args([args])

    println!("cargo:rerun-if-changed={}", current_dir.join("some_dir").to_str().unwrap());
    println!("cargo:rustc-link-search=native=/path/to/lib");
    println!("cargo:rustc-link-lib=static=name");
}