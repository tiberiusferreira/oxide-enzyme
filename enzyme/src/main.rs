use std::ffi::{CStr, CString};
use std::process;
use std::ptr;

use enzyme_sys;
use enzyme_sys::enzyme_ffi::{
    CConcreteType, CFnTypeInfo, CustomRuleType, EnzymeAugmentedReturnPtr,
    EnzymeExtractFunctionFromAugmentation, EnzymeNewTypeTreeCT, EnzymeTypeTreeOnlyEq, IntList,
    LLVMOpaqueContext, LLVMOpaqueModule, LLVMOpaqueValue, CDIFFE_TYPE,
};
use llvm_sys::analysis::{LLVMVerifierFailureAction, LLVMVerifyModule};
use llvm_sys::execution_engine::{LLVMGetFunctionAddress, LLVMLinkInMCJIT};
use llvm_sys::ir_reader::LLVMParseIRInContext;
use llvm_sys::prelude::{LLVMTypeRef, LLVMValueRef};
use llvm_sys::remarks::LLVMRemarkEntryGetFunctionName;
use llvm_sys::target::LLVM_InitializeNativeTarget;
use llvm_sys::target_machine::LLVMGetDefaultTargetTriple;
use llvm_sys::LLVMLinkage::LLVMExternalLinkage;
use llvm_sys::{
    core::*, debuginfo::LLVMGetSubprogram, execution_engine::LLVMCreateExecutionEngineForModule,
    LLVMLinkage,
};
use serde::Deserialize;
use std::io::Read;
use std::ptr::{null, null_mut};

macro_rules! c_str {
    ($s:expr) => {
        concat!($s, "\0").as_ptr() as *const i8
    };
}

pub unsafe fn load_llvm(ir_filepath: String, fun: VerifiedFnToDiff) {
    let context = LLVMContextCreate();
    let mut msg = ptr::null_mut();

    let mut memory_buf = ptr::null_mut();
    let path = CString::new(ir_filepath).unwrap();
    if LLVMCreateMemoryBufferWithContentsOfFile(path.as_ptr(), &mut memory_buf, &mut msg) != 0 {
        panic!("Could not read bitcode!");
    }

    let mut module = ptr::null_mut();
    if LLVMParseIRInContext(context, memory_buf, &mut module, &mut msg) != 0 {
        panic!("Could not create module!");
    }

    if LLVMVerifyModule(
        module,
        LLVMVerifierFailureAction::LLVMReturnStatusAction,
        &mut msg,
    ) != 0
    {
        panic!("Could not validate!");
    }

    // load function
    let diffed_fnc_name = CString::new(fun.diffed_function_name.clone()).unwrap();

    let prev_declaration = LLVMGetNamedFunction(module, diffed_fnc_name.as_ptr());
    if prev_declaration as u8 == 0 {
        panic!("diffed_fnc_name not found");
    }

    let fnc_name = CString::new(fun.function_name.clone()).unwrap();
    let fnc = LLVMGetNamedFunction(module, fnc_name.as_ptr());

    if fnc as usize == 0 {
        panic!("function not found");
    } else {
        println!("Found function");
    }
    let mut name = CString::new("").unwrap().into_raw();
    let mut rules: Vec<CustomRuleType> = vec![];
    let type_analysis = enzyme_sys::enzyme_ffi::CreateTypeAnalysis(
        LLVMGetDefaultTargetTriple(),
        &mut name,
        rules.as_mut_ptr(),
        0,
    );
    let diff = enzyme_sys::AutoDiff::new(module as *mut LLVMOpaqueModule, type_analysis);
    let cxt = LLVMGetModuleContext(module);
    let float_type_left =
        EnzymeNewTypeTreeCT(CConcreteType::DT_Pointer, cxt as *mut LLVMOpaqueContext);
    EnzymeTypeTreeOnlyEq(float_type_left, 0);

    let float_type_right =
        EnzymeNewTypeTreeCT(CConcreteType::DT_Pointer, cxt as *mut LLVMOpaqueContext);
    EnzymeTypeTreeOnlyEq(float_type_right, 0);
    let mut types = vec![float_type_left, float_type_right];
    let types_slice_ptr = types.as_mut_ptr();
    let float_type_ret =
        EnzymeNewTypeTreeCT(CConcreteType::DT_Float, cxt as *mut LLVMOpaqueContext);

    EnzymeTypeTreeOnlyEq(float_type_ret, 0);

    let known = IntList {
        data: null_mut(),
        size: 0,
    };
    let mut known_vec = vec![known, known];
    let fn_type = CFnTypeInfo {
        Arguments: types_slice_ptr,
        Return: float_type_ret,
        KnownValues: known_vec.as_mut_ptr(),
    };
    let b = diff.create_primal_and_grad(
        fnc,
        CDIFFE_TYPE::DFT_OUT_DIFF,
        vec![CDIFFE_TYPE::DFT_DUP_ARG, CDIFFE_TYPE::DFT_DUP_ARG],
        fn_type,
    );
    let max = LLVMCountParams(b);
    for i in 0..max {
        let val = LLVMGetParam(b, i);
        let mut len = 0;
        let name = LLVMGetValueName2(val, &mut len);
        let name_c = CStr::from_ptr(name);
        println!("{:?}", name_c);

        let val = LLVMGetParam(prev_declaration, i);
        LLVMSetValueName2(val, name, len);
    }

    let mut basic_block = LLVMGetEntryBasicBlock(b);
    assert_ne!(basic_block as usize, 0, "basic block was null");
    while basic_block as usize != 0 {
        let next = LLVMGetNextBasicBlock(basic_block);
        LLVMRemoveBasicBlockFromParent(basic_block);
        LLVMAppendExistingBasicBlock(prev_declaration, basic_block);
        basic_block = next;
    }
    LLVMSetLinkage(b, LLVMExternalLinkage);

    let modu = LLVMPrintModuleToString(module);
    let c_str = CString::from_raw(modu);
    let module = c_str.to_string_lossy();
    std::fs::write("diffed.ll", module.as_bytes()).unwrap();

    let context = LLVMContextCreate();
    let mut msg = ptr::null_mut();

    let path = CString::new("diffed.ll").unwrap();
    let mut memory_buf = ptr::null_mut();
    LLVMCreateMemoryBufferWithContentsOfFile(path.as_ptr(), &mut memory_buf, null_mut());

    let mut module = ptr::null_mut();
    if LLVMParseIRInContext(context, memory_buf, &mut module, &mut msg) != 0 {
        println!("{:?}", CString::from_raw(msg));
    }

    if LLVMVerifyModule(
        module,
        LLVMVerifierFailureAction::LLVMAbortProcessAction,
        &mut msg,
    ) != 0
    {
        panic!("Could not validate!");
    }

    println!("Done!");
}

#[derive(Clone, Debug, Deserialize)]
struct Parameters {
    name: String,
    mutable: bool,
}

#[derive(Clone, Debug, Deserialize)]
struct FunctionToDiff {
    name: String,
    parameters: Vec<Parameters>,
}

#[derive(Clone, Debug, Deserialize)]
struct DiffResultFunction {
    diff_result_fn_name: String,
    original_fn_name: String,
    parameters: Vec<Parameters>,
}

#[derive(Clone, Debug)]
pub struct VerifiedFnToDiff {
    function_name: String,
    diffed_function_name: String,
}

pub fn main() {
    // create_example_llvm_ir();
    let mut fn_to_diff: Vec<FunctionToDiff> = vec![];
    let mut diff_result_fn: Vec<DiffResultFunction> = vec![];
    let dir = "../diff";
    let mut llvm_ir_file_path: Option<String> = None;
    let dir = std::fs::read_dir(dir).unwrap();
    for file in dir {
        let file = file.unwrap();
        if file.file_name().to_string_lossy().ends_with(".ll") {
            llvm_ir_file_path = Some(file.path().to_string_lossy().to_string());
        } else if file.file_name().to_string_lossy().ends_with(".json") {
            let mut open_file = std::fs::File::open(file.path()).unwrap();
            let mut content = String::new();
            open_file.read_to_string(&mut content).unwrap();
            if let Ok(function_to_diff) = serde_json::from_str(&content) {
                fn_to_diff.push(function_to_diff);
            } else {
                diff_result_fn.push(serde_json::from_str(&content).unwrap());
            }
        }
    }
    println!("{:#?}", fn_to_diff);
    println!("{:#?}", diff_result_fn);
    println!("{:#?}", llvm_ir_file_path);

    let mut verified = vec![];
    for to_diff in fn_to_diff {
        let result_desired_name = diff_result_fn
            .iter()
            .find(|f| f.original_fn_name == to_diff.name);
        let diff_result =
            result_desired_name.expect(&format!("No matching desired name for {}", to_diff.name));
        assert_eq!(
            diff_result.parameters.len(),
            to_diff.parameters.len() * 2 + 1,
            "Resulting function should have 2x+1 parameters than original"
        );

        verified.push(VerifiedFnToDiff {
            function_name: to_diff.name,
            diffed_function_name: diff_result.diff_result_fn_name.clone(),
        });
    }

    println!("{:#?}", verified);
    let single = verified.pop().unwrap();
    unsafe {
        load_llvm(llvm_ir_file_path.expect("LLVM ir file not found"), single);
    }
}
