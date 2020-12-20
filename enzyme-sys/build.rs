extern crate bindgen;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const LIBRARY_NAME: &'static str = "libenzyme.so";



fn generate_bindings() {
    let header_path = "source/enzyme/Enzyme/CApi.h";

    // tell cargo to re-run the builder if the header has changed
    println!("cargo:rerun-if-changed={}", header_path);

    let bindings = bindgen::Builder::default()
        .header(header_path)
        // add CConcreteType as enum
        .whitelist_type("CConcreteType")
        .rustified_enum("CConcreteType")
        .whitelist_type("CDIFFE_TYPE")
        .rustified_enum("CDIFFE_TYPE")
        .whitelist_type("LLVMContextRef")
        .whitelist_type("CTypeTreeRef")
        .whitelist_type("EnzymeTypeAnalysisRef")
        .whitelist_function("EnzymeNewTypeTree")
        .whitelist_function("EnzymeNewTypeTreeCT")
        .whitelist_function("EnzymeFreeTypeTree")
        .whitelist_function("EnzymeMergeTypeTree")
        .whitelist_function("EnzymeTypeTreeOnlyEq")
        .whitelist_function("EnzymeMergeTypeTree")
        .whitelist_function("EnzymeTypeTreeShiftIndiciesEq")
        .whitelist_function("EnzymeTypeTreeToString")
        .whitelist_function("EnzymeTypeTreeToStringFree")
        .whitelist_function("EnzymeGetGlobalAA")
        .whitelist_function("EnzymeFreeGlobalAA")
        .whitelist_function("EnzymeCreatePrimalAndGradient")
        .whitelist_function("EnzymeCreateAugmentedPrimal")
        .whitelist_function("CreateTypeAnalysis")
        .whitelist_function("FreeTypeAnalysis")
        .whitelist_function("EnzymeExtractReturnInfo")
        .whitelist_function("EnzymeExtractFunctionFromAugmentation")
        .whitelist_function("EnzymeExtractTapeTypeFromAugmentation")
        //.whitelist_type("LLVMOpaqueModule")
        //.whitelist_function("LLVMModuleCreateWithName")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    // let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        // .write_to_file(out_path.join("enzyme.rs"))
        .write_to_file("src/enzyme_bindgen.rs")
        .expect("Couldn't write bindings for enzyme!");
}

fn choose_library() {
    // println!("cargo:rustc-link-lib=dylib=./source/enzyme/build/Enzyme/ClangEnzyme-11.dylib");
    // println!("cargo:rustc-link-lib=dylib=./source/enzyme/build/Enzyme/LLVMEnzyme-11.dylib");
    println!("cargo:rustc-link-search=/Users/tiberio/Documents/github/oxide-c-api/enzyme-sys");
    println!("cargo:rustc-link-search=/Users/tiberio/llvm-11.0.0/lib/");
    println!("cargo:rustc-link-search=/Users/tiberio/llvm-11.0.0/lib/cmake/llvm");
    // println!("cargo:rustc-link-search=native=/Users/tiberio/Documents/github/oxide-c-api/enzyme-sys");
    // println!("cargo:rustc-link-lib=dylib=enzyme");

    println!("cargo:rustc-link-lib=dylib=ClangEnzyme");
    println!("cargo:rustc-link-lib=dylib=LLVMEnzyme");
    // println!("cargo:rustc-link-lib=LLVM-11");
    // println!("cargo:rustc-link-lib=LLVMAnalysis");
    // println!("cargo:rustc-link-lib=LLVMIRReader");
    // println!("cargo:rustc-link-lib=LLVMMIRParser");

    // println!("cargo:rustc-link-lib=framework=LLVMEnzyme-11");
}

fn main() {
    // generate_bindings();
    choose_library();
}
