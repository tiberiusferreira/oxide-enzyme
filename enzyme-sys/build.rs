use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    println!("cargo:rustc-link-search={}", std::env::var("CARGO_MANIFEST_DIR").unwrap());
    println!("cargo:rustc-link-lib=static=LLVMEnzyme-11-static");
}
