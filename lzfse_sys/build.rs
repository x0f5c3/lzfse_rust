extern crate bindgen;
extern crate cmake;


use cmake::Config;

use std::{
    env,
    path::PathBuf,
};


fn main() {
    let dst = Config::new(PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("lzfse"))
        .cflag("/03")
        .cflag("-march=native")
        .cflag("-Wall")
        .cflag("-DNDEBUG")
        .cflag("-D_POSIX_C_SOURCE")
        .cflag("-std=c99")
        .cflag("-fvisibility=hidden")
        .build();
    println!("cargo:rustc-link-search=native={}", dst.join("lib").display());
    println!("cargo:rustc-link-lib=static=lzfse");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .disable_header_comment()
        .rustfmt_bindings(true)
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("src");
    bindings.write_to_file(out_path.join("bindings.rs")).expect("Couldn't write bindings");
}
