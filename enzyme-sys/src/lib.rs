#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod enzyme_ffi;
pub mod tree;
pub mod typeinfo;
use enzyme_ffi::*;
use llvm_sys::prelude::LLVMValueRef;
use llvm_sys::LLVMValue;

pub struct AutoDiff {
    aa_results_ref: EnzymeAAResultsRef,
    type_analysis: EnzymeTypeAnalysisRef,
}

impl AutoDiff {
    pub fn new(module: LLVMModuleRef, type_analysis: EnzymeTypeAnalysisRef) -> AutoDiff {
        let aa_results_ref = unsafe { EnzymeGetGlobalAA(module) };
        AutoDiff {
            aa_results_ref,
            type_analysis,
        }
    }

    pub fn create_augmented_forward(
        &self,
        fnc: LLVMValueRef,
        retType: CDIFFE_TYPE,
        mut args: Vec<CDIFFE_TYPE>,
        args_type_info: CFnTypeInfo,
    ) -> LLVMValueRef {
        // let returned = std::ptr::null_mut();
        return unsafe {
            EnzymeCreateAugmentedPrimal(
                fnc as *mut LLVMOpaqueValue,
                retType,
                args.as_mut_slice().as_mut_ptr(),
                args.len() as u64,
                self.type_analysis,
                self.aa_results_ref,
                1,
                args_type_info,
                [1u8, 1u8].as_mut_ptr(),
                2,
                0,
                0,
                0,
            ) as *mut LLVMValue
        };
    }
    pub fn create_primal_and_grad(
        &self,
        fnc: LLVMValueRef,
        retType: CDIFFE_TYPE,
        mut args: Vec<CDIFFE_TYPE>,
        args_type_info: CFnTypeInfo,
    ) -> LLVMValueRef {
        // let returned = std::ptr::null_mut();
        return unsafe {
            EnzymeCreatePrimalAndGradient(
                fnc as *mut LLVMOpaqueValue,
                retType,
                args.as_mut_slice().as_mut_ptr(),
                args.len() as u64,
                self.type_analysis,
                self.aa_results_ref,
                0,
                0,
                1,
                std::ptr::null_mut(),
                args_type_info,
                [0u8, 0u8].as_mut_ptr(),
                2,
                std::ptr::null_mut(),
                0,
                0,
            ) as *mut LLVMValue
        };
    }
}

impl Drop for AutoDiff {
    fn drop(&mut self) {
        unsafe { EnzymeFreeGlobalAA(self.aa_results_ref) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use llvm_sys::core::{LLVMContextCreate, LLVMModuleCreateWithName};
    use std::ffi::CString;

    #[test]
    fn empty_tree() {
        let _ = unsafe { EnzymeNewTypeTree() };
    }

    #[test]
    fn build_tree() {}

    #[test]
    fn get_global_aa() {
        let dummy_module =
            unsafe { LLVMModuleCreateWithName(CString::new("dummy").unwrap().into_raw()) }
                as *mut LLVMOpaqueModule;

        unsafe {
            let tmp = EnzymeGetGlobalAA(dummy_module);
            EnzymeFreeGlobalAA(tmp);
        }
    }
}
