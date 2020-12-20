use std::ffi::CString;
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
use llvm_sys::target::LLVM_InitializeNativeTarget;
use llvm_sys::target_machine::LLVMGetDefaultTargetTriple;
use llvm_sys::LLVMLinkage::LLVMExternalLinkage;
use llvm_sys::{
    core::*, debuginfo::LLVMGetSubprogram, execution_engine::LLVMCreateExecutionEngineForModule,
};
use std::ptr::null_mut;

pub fn create_example_llvm_ir() {
    if !process::Command::new("rustc")
        .args(&["--emit=llvm-ir", "../example/src/main.rs", "-C debuginfo=2"])
        .output()
        .expect("failed to run rustc")
        .status
        .success()
    {
        panic!("Could not generate sample IR");
    }
}

macro_rules! c_str {
    ($s:expr) => {
        concat!($s, "\0").as_ptr() as *const i8
    };
}

pub unsafe fn load_llvm() {
    let context = LLVMContextCreate();
    let mut msg = ptr::null_mut();

    let mut memory_buf = ptr::null_mut();
    let path = CString::new("./main.ll").unwrap();
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
    let fnc_name = CString::new("_ZN4main10simple_mul17he69584a08c5f72d6E").unwrap();
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
        CDIFFE_TYPE::DFT_CONSTANT,
        vec![CDIFFE_TYPE::DFT_DUP_ARG, CDIFFE_TYPE::DFT_DUP_ARG],
        fn_type,
    );
    // let fun = EnzymeExtractFunctionFromAugmentation(b as EnzymeAugmentedReturnPtr) as LLVMValueRef;

    LLVMDumpValue(b);
    //
    // let modu = LLVMPrintModuleToString(module);
    // let modu = CString::from_raw(modu).into_string().unwrap();
    // println!("{}", modu);
    // println!("{:?}", w);
}

pub fn main() {
    // create_example_llvm_ir();
    unsafe {
        load_llvm();
    }
}
