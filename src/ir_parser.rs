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

    let mut local_types_with_metadata: HashMap<LLVMLocalTypeVariableDebugInfo, DILocalVariable>  = HashMap::new();
    let mut local_types_with_basic_types: HashMap<LLVMLocalTypeVariableDebugInfo, BasicTypeOrCompositeType>  = HashMap::new();
    for val in &ir.llvm_local_type_variable_debug_info{
        let debug_info = ast.inner.get(&val.location_tag).expect(&format!("No debug info for: {:?}", val));
        if let TypeAST::DILocalVariable(local) = debug_info {
            let mut result = Vec::new();
            local_var_unroll_into_basic_types(local, &mut result);
            assert_eq!(result.len(), 1);
            // println!("{:?} => {:#?}", local.name , result);
            local_types_with_metadata.insert(val.clone(), local.clone());
            local_types_with_basic_types.insert(val.clone(), result.pop().unwrap());
        }else{
            panic!("Debug info for local var was not of type for local var!");
        }
    }

    println!("{:#?}", local_types_with_basic_types);


}

#[derive(Clone, Debug)]
pub enum BasicTypeOrCompositeType {
    BasicType(BasicType),
    BasicPointerType(BasicPointerType),
    Composite(String, Vec<BasicTypeOrCompositeType>)
}

// #[derive(Clone, Debug)]
// pub enum BasicTypes{
//     BasicType(BasicType),
//     BasicPointerType(BasicPointerType)
// }

#[derive(Clone, Debug)]
pub struct BasicPointerType{
    pub original: DIBasicType,
}

#[derive(Clone, Debug)]
pub struct BasicType{
    pub name_if_derived: Option<String>,
    pub original: DIBasicType,
}

pub fn local_var_unroll_into_basic_types<'a>(var: &DILocalVariable, result: &'a mut Vec<BasicTypeOrCompositeType>) -> &'a mut Vec<BasicTypeOrCompositeType>{
    match &var.r#type{
        LocalVariableTypes::DIDerivedType(der_var) => {
            match der_var.base_type.as_ref(){
                BaseType::DIDerivedType(derived) => {
                    derived_type_unroll_into_basic_types(derived, result)
                }
                BaseType::DIBasicType(basic) => {
                    result.push(BasicTypeOrCompositeType::BasicType(BasicType{
                        name_if_derived: None,
                        original: basic.clone()
                    }));
                    result
                }
                BaseType::DICompositeType(comp) => composite_type_unroll_into_basic_types(comp, result),
                BaseType::DISubroutineType(_) => {unimplemented!()}
            }
        }
        LocalVariableTypes::DIBasicType(basic) => {
            result.push(BasicTypeOrCompositeType::BasicType(BasicType{
                name_if_derived: None,
                original: basic.clone()
            }));
            result
        }
        LocalVariableTypes::DICompositeType(comp) => composite_type_unroll_into_basic_types(comp, result)
    }
}

pub fn derived_type_unroll_into_basic_types<'a>(var: &DIDerivedType, result: &'a mut Vec<BasicTypeOrCompositeType>) -> &'a mut Vec<BasicTypeOrCompositeType>{
    match var.base_type.as_ref(){
        BaseType::DIDerivedType(derived) => derived_type_unroll_into_basic_types(derived, result),
        BaseType::DIBasicType(basic) => {
            if var.tag == "DW_TAG_pointer_type"{
                result.push(BasicTypeOrCompositeType::BasicPointerType(BasicPointerType{
                    original: basic.clone()
                }));
            }else{
                result.push(BasicTypeOrCompositeType::BasicType(BasicType{
                    name_if_derived: var.name.clone(),
                    original: basic.clone()
                }))
            }

            result
        }
        BaseType::DICompositeType(comp) => composite_type_unroll_into_basic_types(comp, result),
        BaseType::DISubroutineType(_) => unimplemented!()
    }
}

pub fn composite_type_unroll_into_basic_types<'a>(var: &DICompositeType, result: &'a mut Vec<BasicTypeOrCompositeType>) -> &'a mut Vec<BasicTypeOrCompositeType>{
    let mut new_vec = vec![];
    for el in &var.elements{
        match el{
            CompositeTypeElements::DIDerivedType(de) => {
                derived_type_unroll_into_basic_types(de, &mut new_vec);
            }
            CompositeTypeElements::DICompositeType(comp) => {
                composite_type_unroll_into_basic_types(comp, &mut new_vec);
            },
            CompositeTypeElements::DIEnumerator(_) => {}
            CompositeTypeElements::DISubrange(_) => {}
        }
    }
    let new_curr = BasicTypeOrCompositeType::Composite(var.identifier.clone().unwrap_or_default(), new_vec);
    result.push(new_curr);
    result
}

// --- Draft for unwrapping Composite types
/*
  For structs and Composite types in general (minus arrays) it seems that we want
  the getelementptr instruction https://llvm.org/docs/LangRef.html#getelementptr-instruction
  store float 0x3FC99999A0000000, float* %b, align 4, !dbg !1177
  call void @llvm.dbg.value(metadata float* %b, metadata !1178, metadata !DIExpression()) #31, !dbg !1184
  %_3.i = ptrtoint float* %b to i64, !dbg !1186
  call void @__enzyme_float(i64 %_3.i, i64 4) #31, !dbg !1187
*/


/*
LLVMLocalTypeVariableDebugInfo {
        local_var_type: "%Tensor*",
        local_var_name: "%linear_weights",
        location_tag: "!442",
    }: DILocalVariable {
        file: "!396",
        name: Some(
            "\"self\"",
        ),
        arg: Some(
            "1",
        ),
        line: "820",
        type: DIDerivedType(
            DIDerivedType {
                tag: "DW_TAG_pointer_type",
                align: "64",
                dwarf_address_space: Some(
                    "0",
                ),
                size: Some(
                    "64",
                ),
                name: Some(
                    "\"&alloc::vec::Vec<f64>\"",
                ),
                base_type: DICompositeType(
                    DICompositeType {
                        tag: "DW_TAG_structure_type",
                        identifier: Some(
                            "\"8ec7b5be661549003945426fa3a752ac\"",
                        ),
                        elements: [
                            DIDerivedType(
                                DIDerivedType {
                                    tag: "DW_TAG_member",
                                    align: "64",
                                    dwarf_address_space: None,
                                    size: Some(
                                        "128",
                                    ),
                                    name: Some(
                                        "\"buf\"",
                                    ),
                                    base_type: DICompositeType(
                                        DICompositeType {
                                            tag: "DW_TAG_structure_type",
                                            identifier: Some(
                                                "\"fc1292fdd82e1afcfe9891a69b262a4b\"",
                                            ),
                                            elements: [
                                                DIDerivedType(
                                                    DIDerivedType {
                                                        tag: "DW_TAG_member",
                                                        align: "64",
                                                        dwarf_address_space: None,
                                                        size: Some(
                                                            "64",
                                                        ),
                                                        name: Some(
                                                            "\"ptr\"",
                                                        ),
                                                        base_type: DICompositeType(
                                                            DICompositeType {
                                                                tag: "DW_TAG_structure_type",
                                                                identifier: Some(
                                                                    "\"fd0e448f54d1412c9c149e206a4d797\"",
                                                                ),
                                                                elements: [
                                                                    DIDerivedType(
                                                                        DIDerivedType {
                                                                            tag: "DW_TAG_member",
                                                                            align: "64",
                                                                            dwarf_address_space: None,
                                                                            size: Some(
                                                                                "64",
                                                                            ),
                                                                            name: Some(
                                                                                "\"pointer\"",
                                                                            ),
                                                                            base_type: DIDerivedType(
                                                                                DIDerivedType {
                                                                                    tag: "DW_TAG_pointer_type",
                                                                                    align: "64",
                                                                                    dwarf_address_space: Some(
                                                                                        "0",
                                                                                    ),
                                                                                    size: Some(
                                                                                        "64",
                                                                                    ),
                                                                                    name: Some(
                                                                                        "\"*const f64\"",
                                                                                    ),
                                                                                    base_type: DIBasicType(
                                                                                        DIBasicType {
                                                                                            encoding: "DW_ATE_float",
                                                                                            size: Some(
                                                                                                "64",
                                                                                            ),
                                                                                            name: "\"f64\"",
                                                                                        },
                                                                                    ),
                                                                                },
                                                                            ),
                                                                        },
                                                                    ),
                                                                    DIDerivedType(
                                                                        DIDerivedType {
                                                                            tag: "DW_TAG_member",
                                                                            align: "8",
                                                                            dwarf_address_space: None,
                                                                            size: None,
                                                                            name: Some(
                                                                                "\"_marker\"",
                                                                            ),
                                                                            base_type: DICompositeType(
                                                                                DICompositeType {
                                                                                    tag: "DW_TAG_structure_type",
                                                                                    identifier: Some(
                                                                                        "\"70467b90512e64f071c1cde9f649e4a\"",
                                                                                    ),
                                                                                    elements: [],
                                                                                    vtable_holder: None,
                                                                                    flags: None,
                                                                                    name: Some(
                                                                                        "\"PhantomData<f64>\"",
                                                                                    ),
                                                                                    file: Some(
                                                                                        "!8",
                                                                                    ),
                                                                                    align: "8",
                                                                                },
                                                                            ),
                                                                        },
                                                                    ),
                                                                ],
                                                                vtable_holder: None,
                                                                flags: None,
                                                                name: Some(
                                                                    "\"Unique<f64>\"",
                                                                ),
                                                                file: Some(
                                                                    "!8",
                                                                ),
                                                                align: "64",
                                                            },
                                                        ),
                                                    },
                                                ),
                                                DIDerivedType(
                                                    DIDerivedType {
                                                        tag: "DW_TAG_member",
                                                        align: "64",
                                                        dwarf_address_space: None,
                                                        size: Some(
                                                            "64",
                                                        ),
                                                        name: Some(
                                                            "\"cap\"",
                                                        ),
                                                        base_type: DIBasicType(
                                                            DIBasicType {
                                                                encoding: "DW_ATE_unsigned",
                                                                size: Some(
                                                                    "64",
                                                                ),
                                                                name: "\"usize\"",
                                                            },
                                                        ),
                                                    },
                                                ),
                                                DIDerivedType(
                                                    DIDerivedType {
                                                        tag: "DW_TAG_member",
                                                        align: "8",
                                                        dwarf_address_space: None,
                                                        size: None,
                                                        name: Some(
                                                            "\"alloc\"",
                                                        ),
                                                        base_type: DICompositeType(
                                                            DICompositeType {
                                                                tag: "DW_TAG_structure_type",
                                                                identifier: Some(
                                                                    "\"4e639e0977d2a7dfbf4c96ed56fb6695\"",
                                                                ),
                                                                elements: [],
                                                                vtable_holder: None,
                                                                flags: None,
                                                                name: Some(
                                                                    "\"Global\"",
                                                                ),
                                                                file: Some(
                                                                    "!8",
                                                                ),
                                                                align: "8",
                                                            },
                                                        ),
                                                    },
                                                ),
                                            ],
                                            vtable_holder: None,
                                            flags: None,
                                            name: Some(
                                                "\"RawVec<f64",
                                            ),
                                            file: Some(
                                                "!8",
                                            ),
                                            align: "64",
                                        },
                                    ),
                                },
                            ),
                            DIDerivedType(
                                DIDerivedType {
                                    tag: "DW_TAG_member",
                                    align: "64",
                                    dwarf_address_space: None,
                                    size: Some(
                                        "64",
                                    ),
                                    name: Some(
                                        "\"len\"",
                                    ),
                                    base_type: DIBasicType(
                                        DIBasicType {
                                            encoding: "DW_ATE_unsigned",
                                            size: Some(
                                                "64",
                                            ),
                                            name: "\"usize\"",
                                        },
                                    ),
                                },
                            ),
                        ],
                        vtable_holder: None,
                        flags: None,
                        name: Some(
                            "\"Vec<f64>\"",
                        ),
                        file: Some(
                            "!8",
                        ),
                        align: "64",
                    },
                ),
            },
        ),
        scope: "!438",
    },
*/

/*
Some("\"linear_weights\"") => [
    DIBasicType {
        encoding: "DW_ATE_float",
        size: Some(
            "64",
        ),
        name: "\"f64\"",
    },
    DIBasicType {
        encoding: "DW_ATE_unsigned",
        size: Some(
            "64",
        ),
        name: "\"usize\"",
    },
    DIBasicType {
        encoding: "DW_ATE_unsigned",
        size: Some(
            "64",
        ),
        name: "\"usize\"",
    },
]
*/