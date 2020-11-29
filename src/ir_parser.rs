use regex::Regex;
use std::collections::HashMap;
use crate::structs::*;
use crate::file_parser::*;

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

fn get_rust_debug_metadata(tag: &str, metadata_vec: &[LLVMDebugTypeInformation]) -> Option<LLVMDebugTypeInformation>{
    metadata_vec
        .iter()
        .find(|a| a.location_tag == tag).cloned()
}


struct LocalVarType{
    nested_types: Vec<String>
}
enum LocalVarTypeOrComposite{
    Composite(Vec<String>),
    LocalVarType(LocalVarType)
}

struct LocalVarTypeTree{
    name: String
}

///
fn describe_local_var(value: &LLVMDebugTypeInformation, ir: &LLVMIRMetadata){
    let rust_metadata: &[LLVMDebugTypeInformation] = &ir.llvm_debug_type_information;
    println!("Getting local var type for {:#?}", value);
    LocalVarTypeTree{
        name: value.get_name().expect("Local var had no name")
    };
    let variable_type = get_rust_debug_metadata(&value.parameters.get("type").unwrap(), rust_metadata).unwrap();
    // let type_name = variable_type.parameters.get("name").expect("Type had no name");
    // let output = LocalVarType{nested_types: vec!["Pointer!".to_string()]};
    //

    if let Some(tag) = variable_type.get_tag(){
        if matches!(tag, Tag::TagPointerType){
            println!("Pointer!");
            // return LocalVarTypeOrComposite::LocalVarType(LocalVarType{nested_types: vec!["Pointer!".to_string()]});
            return ;
        }
    }
    if let Some(variant) = variable_type.get_variant(){
        match variant{
            Variant::LocalVariable => {}
            Variant::SubroutineType => {}
            Variant::DerivedType => {}
            Variant::BasicType => {
                println!("BasicType! Encoding: {} Size: {} ", variable_type.parameters.get("encoding").unwrap(),
                         variable_type.parameters.get("size").unwrap());
                return;
            }
            Variant::CompositeType => {
                println!("CompositeType!");
                match variable_type.get_tag() {
                    None => {
                        println!("No tag for CompositeType!");
                        println!("{:?}", variable_type);
                        panic!()
                    }
                    Some(tag) => {
                        match tag{
                            Tag::TagPointerType => {}
                            Tag::TagStructureType => {
                                println!("Structure type!");
                                let elements = variable_type.parameters.get("elements").expect("No elements tag!");
                                let elements_tags = ir.multiple_tags_tag.get(elements).unwrap();
                                return;
                                // for el in elements_tags{
                                //     let dbg = get_rust_debug_metadata(el, &ir.llvm_debug_type_information).unwrap();
                                //     describe_local_var(&dbg, &ir);
                                // }
                                // get_rust_debug_metadata(elements, rust_metadata).unwrap();
                            }
                            Tag::TagMember => {}
                            Tag::TagVariantPart => {}
                            Tag::TagArrayType => {
                                println!("Array Type!");
                                if let Some(base_type) = variable_type.parameters.get("baseType"){
                                    println!("Base Type = {}", base_type);
                                }else{
                                    panic!("Not base type for array!");
                                }
                            }
                        }
                    }
                }
                return;
            }
            Variant::TemplateTypeParameter => {}
            Variant::Namespace => {}
            Variant::File => {}
            Variant::GlobalVariableExpression => {}
            Variant::Enumerator => {}
            Variant::DISubrange => {}
            Variant::DILocation => {}
            _ => {}
        }
    }
    println!("{:?}", variable_type);
    panic!("Local variable type has not tag!");
}


pub fn main(){
    let ir = LLVMIRMetadata::new("oxide_enzyme_replaced.ll");

    for local_llvm_type in ir.llvm_debug_type_information.iter().take(5000) {
        if matches!(local_llvm_type.get_variant().expect(&format!("No Variant for {:?}", local_llvm_type)), Variant::LocalVariable | Variant::BasicType | Variant::DerivedType){
            parse_llvm_debug_type_information(&local_llvm_type);
        }

        // if let Some(variant) = single_rust_metadata.get_variant(){
        //     if matches!(variant, Variant::LocalVariable){
        //         println!("Is Local Variable");
        //         describe_local_var(&single_rust_metadata, &ir);
        //         println!();
        //         println!();
        //         println!();
        //         println!();
        //     }
        // }
    }

    // for local_llvm_type in ir.llvm_local_type_variable_debug_info.iter().take(5000) {
    //     // println!("Investigating: {:#?}", local_llvm_type);
    //     let single_rust_metadata = get_rust_debug_metadata(&local_llvm_type.location_tag, &ir.llvm_debug_type_information).unwrap();
    //     // println!("{:?}", &single_rust_metadata);
    //     if matches!(single_rust_metadata.get_variant().expect("No Variant"), Variant::LocalVariable | Variant::BasicType | Variant::DerivedType){
    //         parse_llvm_debug_type_information(&single_rust_metadata);
    //     }
    //
    //     // if let Some(variant) = single_rust_metadata.get_variant(){
    //     //     if matches!(variant, Variant::LocalVariable){
    //     //         println!("Is Local Variable");
    //     //         describe_local_var(&single_rust_metadata, &ir);
    //     //         println!();
    //     //         println!();
    //     //         println!();
    //     //         println!();
    //     //     }
    //     // }
    // }


}
