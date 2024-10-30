use bindgen;
use std::{
    env,
    path::Path,
    process::Command,
};

fn main() {
    let c_src_file_dir = Path::new("ksw2_c_src");
    let current_dir = env::current_dir().unwrap();
    let bindings = bindgen::Builder::default()
        .header(c_src_file_dir.join("ksw2.h").to_str().unwrap())
        .clang_arg("-DHAVE_KALLOC")
        // .clang_arg(&format!("-I{}", c_src_file_dir.to_str().unwrap()))
        .allowlist_function("ksw_.*")
        .allowlist_function("km_.*")
        .generate()
        .expect("generate binding error");

    bindings
        .write_to_file(current_dir.join("src").join("ksw2_sys.rs"))
        .expect("binding write to file error");

    Command::new("cp")
        .current_dir(c_src_file_dir)
        .arg("Makefile")
        .arg("Makefile.old")
        .output()
        .expect("Failed to backup ksw2 makefile.");

    Command::new("sed")
        .current_dir(c_src_file_dir)
        .arg("-i")
        .arg("s/-g -Wall -Wextra -Wc++-compat -O2/ -g -Wall -Wextra -Wc++-compat -O2 -fPIC/g")
        .arg("Makefile")
        .output()
        .expect("Failed to modify ksw2 makefile.");

    // // OUT_DIR is the build output dir
    // let build_out_path = Path::new(&env::var("OUT_DIR").unwrap());

    Command::new("make")
        .current_dir(c_src_file_dir.to_str().unwrap())
        .output()
        .expect("build error");

    Command::new("sh")
        .arg("-c")
        .arg("ar rcs libksw2.a ksw2_*.o kalloc.o")
        .current_dir(c_src_file_dir)
        .output()
        .expect("package libksw2.a error");

    Command::new("cp")
        .current_dir(c_src_file_dir)
        .arg("Makefile.old")
        .arg("Makefile")
        .output()
        .expect("Failed to backup ksw2 makefile.");

    Command::new("sh")
        .arg("-c")
        .arg("rm *.o ksw2-test Makefile.old")
        .current_dir(c_src_file_dir)
        .output()
        .expect("clean error");

    println!(
        "cargo:rerun-if-changed={}",
        current_dir.join("some_dir").to_str().unwrap()
    );
    println!("cargo:rustc-link-search=native={}", c_src_file_dir.to_str().unwrap());
    println!("cargo:rustc-link-lib=static=ksw2");
}
