
fn main() {
    let header_path = "source/enzyme/Enzyme/CApi.h";
    let bindings = bindgen::Builder::default()
        .header(header_path)
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
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("src/enzyme_bindgen.rs")
        .expect("Couldn't write bindings for enzyme!");
}