//! aw
//!
use regex::Regex;
use std::collections::HashMap;
use crate::structs::*;

mod structs;
mod file_parser;
mod ir_transverser;

pub struct LLVMIRMetadata{
    pub multiple_tags_tag: HashMap<String, Vec<String>>,
    pub llvm_local_type_variable_debug_info: Vec<LLVMLocalTypeVariableDebugInfo>,
    pub llvm_debug_type_information: Vec<LLVMDebugTypeInformation>
}

/// Extracts `!1485, !1489` into a `vec!["!1485", "!1489"]`
fn extract_llvm_multiple_tags(tags: &str) -> Vec<String>{
    let tag_regex = r#"(!\d+)"#;
    let regex = Regex::new(tag_regex).unwrap();
    let mut tags_vec = vec![];
    for caps in regex.captures_iter(tags) {
        assert_eq!(caps.len(), 2);
        tags_vec.push(caps[1].to_string());
    }
    tags_vec
}

/// Turns strings such as `tag: DwTagPointerType, name: "&mut alloc::vec::Vec<f64>", baseType: !6`
/// into a `tag` => `DwTagPointerType`, `name` => `&mut alloc::vec::Vec<f64>` etc HashMap
fn get_all_params(input: String) -> HashMap<String, String>{
    let debug_value_description = r#"([^:\s]+):\s([^,]+)[,\s]?"#;
    let re = Regex::new(debug_value_description).unwrap();
    let mut debug_metadatas: HashMap<String, String> = HashMap::new();
    for caps in re.captures_iter(&input){
        debug_metadatas.insert(caps[1].to_string(), caps[2].to_string());
    }
    debug_metadatas
}

pub fn get_rust_debug_metadata(tag: &str, metadata_vec: &[LLVMDebugTypeInformation]) -> Option<LLVMDebugTypeInformation>{
    metadata_vec
        .iter()
        .find(|a| a.location_tag == tag).cloned()
}




pub fn main(){
    let ir = LLVMIRMetadata::new("oxide_enzyme_replaced.ll");
    let mut ast = Ast{
        inner: HashMap::new()
    };
    for local_llvm_type in &ir.llvm_debug_type_information {
        if matches!(local_llvm_type.get_variant().expect(&format!("No Variant for {:?}", local_llvm_type)), Variant::LocalVariable | Variant::BasicType | Variant::DerivedType | Variant::CompositeType | Variant::TemplateTypeParameter | Variant::DISubroutineType){
            let parsed_variant = parse_llvm_debug_type_information(&local_llvm_type, &ir.llvm_debug_type_information, &ir.multiple_tags_tag);
            ast.inner.insert(local_llvm_type.location_tag.clone(), parsed_variant);
        }
    }

    for (_location_tag, ast_type) in ast.inner{
        if let TypeAST::DILocalVariable(var) = ast_type{
            println!("{:#?}", var);
        }
    }

}
