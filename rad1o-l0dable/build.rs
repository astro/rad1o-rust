use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

const LINKER_SCRIPT: &str = &"l0dable.x";

fn main() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join(LINKER_SCRIPT))
        .unwrap()
        .write_all(include_bytes!("l0dable.x"))  // LINKER_SCRIPT
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed={}", LINKER_SCRIPT);
    // println!("cargo:rustc-linker=lld");
    // println!("cargo:rustc-linker-flavor=ld.lld");
    // println!("cargo:rustc-link-arg=-T{}", LINKER_SCRIPT);

    println!("cargo:rerun-if-changed=build.rs");
}
