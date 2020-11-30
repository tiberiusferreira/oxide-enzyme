use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use crate::get_rust_debug_metadata;

/// This corresponds to a link between the LLVM local variable and the debug information
/// from a line such as:
/// `call void @llvm.dbg.value(metadata %"std::fmt::Formatter"* %f, metadata !214, metadata !DIExpression()), !dbg !217`
#[derive(Debug, Clone)]
pub struct LLVMLocalTypeVariableDebugInfo {
    /// In the example above: `%Tensor`
    pub local_var_type: String,
    /// In the example above: `%left`
    pub local_var_name: String,
    /// In the example above: `!1323`
    pub location_tag: String,
}

/// This corresponds to the Rust Metadata information, such as
/// !1298 = !DIDerivedType(tag: DwTagPointerType, name: "&mut alloc::vec::Vec<f64>", baseType: !6, size: 64, align: 64, dwarfAddressSpace: 0)
#[derive(Debug, Clone)]
pub struct LLVMDebugTypeInformation {
    /// In the example above: `!1298`
    pub location_tag: String,
    /// In the example above: `!DIDerivedType`
    pub variant: String,
    /// A HashMap of each name:value, such as `tag` -> `DwTagPointerType`
    pub parameters: HashMap<String, String>,
}

#[derive(Clone, Debug)]
pub struct DILocalVariable{
    pub file: String,
    pub name: Option<String>,
    pub arg: Option<String>,
    pub line: String,
    pub r#type: LocalVariableTypes,
    pub scope: String,
}

#[derive(Clone, Debug)]
pub enum LocalVariableTypes{
    DIDerivedType(DIDerivedType),
    DIBasicType(DIBasicType),
    DICompositeType(DICompositeType)
}

impl From<TypeAST> for LocalVariableTypes{
    fn from(ast_type: TypeAST) -> Self {
        match ast_type{
            TypeAST::DerivedType(t) => LocalVariableTypes::DIDerivedType(t),
            TypeAST::DIBasicType(t) => LocalVariableTypes::DIBasicType(t),
            TypeAST::DICompositeType(t) => LocalVariableTypes::DICompositeType(t),
            _ => panic!("Invalid type for local var type")
        }
    }
}


#[derive(Clone, Debug)]
pub struct DIDerivedType{
    tag: String,
    align: String,
    dwarf_address_space: Option<String>,
    size: Option<String>,
    name: Option<String>,
    base_type: Box<BaseType>,
}

#[derive(Clone, Debug)]
pub struct DIBasicType{
    encoding: String,
    size: Option<String>,
    name: String,
}

#[derive(Clone, Debug)]
pub struct DICompositeType{
    tag: String,
    identifier: Option<String>,
    elements: Vec<CompositeTypeElements>, // DerivedType, DISubrange, Enumerator, CompositeType
    vtable_holder: Option<String>,
    flags: Option<String>,
    name: Option<String>,
    file: Option<String>,
    align: String,
}

#[derive(Clone, Debug)]
pub enum CompositeTypeElements{
    DIDerivedType(DIDerivedType),
    DICompositeType(DICompositeType),
    DIEnumerator(DIEnumerator),
    DISubrange(DISubrange)
}

impl From<TypeAST> for CompositeTypeElements{
    fn from(ast_type: TypeAST) -> Self {
        match ast_type{
            TypeAST::DerivedType(t) => CompositeTypeElements::DIDerivedType(t),
            TypeAST::DICompositeType(t) => CompositeTypeElements::DICompositeType(t),
            TypeAST::DIEnumerator(t) => CompositeTypeElements::DIEnumerator(t),
            TypeAST::DISubrange(t) => CompositeTypeElements::DISubrange(t),
            _ => panic!("Invalid type for BaseType")
        }
    }
}



#[derive(Clone, Debug)]
pub struct DITemplateTypeParameter{
    name: String,
    r#type: String,
}

#[derive(Clone, Debug)]
pub struct DISubroutineType{
    types: String
}

#[derive(Clone, Debug)]
pub struct DIEnumerator{
    name: String,
    value: String
}

#[derive(Clone, Debug)]
pub struct DISubrange{
    count: String,
    lower_bound: String
}

#[derive(Debug, Clone)]
pub enum TypeAST{
    DILocalVariable(DILocalVariable),
    DerivedType(DIDerivedType),
    DIBasicType(DIBasicType),
    DICompositeType(DICompositeType),
    DISubroutineType(DISubroutineType),
    DITemplateTypeParameter(DITemplateTypeParameter),
    DIEnumerator(DIEnumerator),
    DISubrange(DISubrange)
}

#[derive(Debug, Clone)]
pub enum BaseType{
    DIDerivedType(DIDerivedType),
    DIBasicType(DIBasicType),
    DICompositeType(DICompositeType),
    DISubroutineType(DISubroutineType),
}

impl From<TypeAST> for BaseType{
    fn from(ast_type: TypeAST) -> Self {
        match ast_type{
            TypeAST::DerivedType(t) => BaseType::DIDerivedType(t),
            TypeAST::DIBasicType(t) => BaseType::DIBasicType(t),
            TypeAST::DICompositeType(t) => BaseType::DICompositeType(t),
            TypeAST::DISubroutineType(t) => BaseType::DISubroutineType(t),
            _ => panic!("Invalid type for BaseType")
        }
    }
}

#[derive(Debug, Clone)]
pub struct Ast{
    pub inner: HashMap<String, TypeAST>
}


pub fn parse_llvm_debug_type_information(debug_type_info: &LLVMDebugTypeInformation, full_debug_type_info: &[LLVMDebugTypeInformation], multiple_tags_tag: &HashMap<String, Vec<String>>) -> TypeAST{
    return match debug_type_info.get_variant().expect("No Variant"){
        Variant::LocalVariable => {
        let get_val = |field: &str| debug_type_info.parameters.get(field).expect(&format!("No {} in DILocalVariable", field)).clone();
        let get_val_optional = |field: &str| debug_type_info.parameters.get(field).cloned();
            let local_type = get_rust_debug_metadata(&get_val("type"),full_debug_type_info).expect("Invalid tag");
            let local_var_type = LocalVariableTypes::from(parse_llvm_debug_type_information(&local_type, full_debug_type_info, multiple_tags_tag));
            TypeAST::DILocalVariable(DILocalVariable{
                file: get_val("file"),
                name: get_val_optional("name"),
                arg: get_val_optional("arg"),
                line: get_val("line"),
                r#type: local_var_type,
                scope: get_val("scope")
            })
        }
        Variant::DISubroutineType => {
            let get_val = |field: &str| debug_type_info.parameters.get(field).expect(&format!("No {} in DISubroutineType", field)).clone();
            TypeAST::DISubroutineType(DISubroutineType{
                types: get_val("types")
            })
        }
        Variant::DerivedType => {
            let get_val = |field: &str| debug_type_info.parameters.get(field).expect(&format!("No {} in DIDerivedType", field)).clone();
            let get_val_optional = |field: &str| debug_type_info.parameters.get(field).cloned();
            let base_type = get_rust_debug_metadata(&get_val("baseType"),full_debug_type_info).expect("Invalid tag");
            TypeAST::DerivedType(DIDerivedType{
                tag: get_val("tag"),
                align: get_val("align"),
                dwarf_address_space: get_val_optional("dwarfAddressSpace"),
                size: get_val_optional("size"),
                name: get_val_optional("name"),
                base_type: Box::new(BaseType::from(parse_llvm_debug_type_information(&base_type, full_debug_type_info, multiple_tags_tag)))
            })
        }
        Variant::BasicType => {
            let get_val = |field: &str| debug_type_info.parameters.get(field).expect(&format!("No {} in BasicType", field)).clone();
            let get_val_optional = |field: &str| debug_type_info.parameters.get(field).cloned();
            TypeAST::DIBasicType(DIBasicType{
                encoding: get_val("encoding"),
                size: get_val_optional("size"),
                name: get_val("name"),
            })
        }
        Variant::CompositeType => {
            let get_val = |field: &str| debug_type_info.parameters.get(field).expect(&format!("No {} in CompositeType", field)).clone();
            let get_val_optional = |field: &str| debug_type_info.parameters.get(field).cloned();
            let els = multiple_tags_tag.get(&get_val("elements")).expect("Invalid tag for tags");
            let mut composite_els = vec![];
            for el in els{
                let a = get_rust_debug_metadata(el, full_debug_type_info).expect("Invalid tag");
                let b = parse_llvm_debug_type_information(&a, full_debug_type_info, multiple_tags_tag);
                composite_els.push(CompositeTypeElements::from(b))
            }
            // println!("{:?}", els);
            TypeAST::DICompositeType(DICompositeType{
                tag: get_val("tag"),
                identifier: get_val_optional("identifier"),
                elements: composite_els,
                vtable_holder: get_val_optional("vtableHolder"),
                flags: get_val_optional("flags"),
                name: get_val_optional("name"),
                file: get_val_optional("file"),
                align: get_val("align")
            })
        }
        Variant::TemplateTypeParameter => {
            let get_val = |field: &str| debug_type_info.parameters.get(field).expect(&format!("No {} in CompositeType", field)).clone();
            TypeAST::DITemplateTypeParameter(DITemplateTypeParameter{
                name: get_val("name"),
                r#type: get_val("type")
            })
        }
        Variant::Namespace => {unimplemented!()}
        Variant::File => {unimplemented!()}
        Variant::GlobalVariableExpression => {unimplemented!()}
        Variant::Enumerator => {
            let get_val = |field: &str| debug_type_info.parameters.get(field).expect(&format!("No {} in CompositeType", field)).clone();
            TypeAST::DIEnumerator(DIEnumerator{
                name: get_val("name"),
                value: get_val("value")
            })
        }
        Variant::DISubrange => {
            let get_val = |field: &str| debug_type_info.parameters.get(field).expect(&format!("No {} in CompositeType", field)).clone();
            TypeAST::DISubrange(DISubrange{
                count: get_val("count"),
                lower_bound: get_val("lowerBound")
            })
        }
        Variant::DILocation => {unimplemented!()}
        _ => {unimplemented!()}
    }
}


/// This corresponds to the Rust Metadata information, such as
/// !1488 = !{!1485, !1489}
#[derive(Debug, Clone)]
pub struct MultipleTagsTag{
    pub location_tag: String,
    pub tags: Vec<String>
}

#[derive(Clone, Debug)]
pub enum Variant{
    LocalVariable,
    DISubroutineType,
    DerivedType,
    DILocation,
    DISubrange,
    DILexicalBlockFile,
    BasicType,
    CompositeType,
    TemplateTypeParameter,
    Namespace,
    File,
    GlobalVariableExpression,
    Enumerator,
}

#[derive(Clone, Debug)]
pub enum Tag{
    TagPointerType,
    TagStructureType,
    TagMember,
    TagVariantPart,
    TagArrayType,
}

impl TryFrom<&str> for Tag{
    type Error = ();
    fn try_from(variant_str: &str) -> Result<Self, Self::Error> {
        match variant_str{
            "DwTagPointerType" => Ok(Self::TagPointerType),
            "DwTagStructureType" => Ok(Self::TagStructureType),
            "DwTagMember" => Ok(Self::TagMember),
            "DwTagVariantPart" => Ok(Self::TagVariantPart),
            "DW_TAG_array_type" => Ok(Self::TagArrayType),
            _ => Err(()),
        }
    }
}

impl TryFrom<&str> for Variant{
    type Error = ();
    fn try_from(variant_str: &str) -> Result<Self, Self::Error> {
        match variant_str{
            "!DILocalVariable" => Ok(Self::LocalVariable),
            "!DISubroutineType" => Ok(Self::DISubroutineType),
            "!DIDerivedType" => Ok(Self::DerivedType),
            "!DIBasicType" => Ok(Self::BasicType),
            "!DISubrange" => Ok(Self::DISubrange),
            "!DILexicalBlockFile" => Ok(Self::DILexicalBlockFile),
            "!DILocation" => Ok(Self::DILocation),
            "!DICompositeType" => Ok(Self::CompositeType),
            "!DITemplateTypeParameter" => Ok(Self::TemplateTypeParameter),
            "!DINamespace" => Ok(Self::Namespace),
            "!DIFile" => Ok(Self::File),
            "!DIGlobalVariableExpression" => Ok(Self::GlobalVariableExpression),
            "!DIEnumerator" => Ok(Self::Enumerator),
            _ => Err(()),
        }
    }
}

impl LLVMDebugTypeInformation {
    pub fn get_variant(&self) -> Option<Variant>{
        self.variant.as_str().try_into().ok()
    }
    pub fn get_tag(&self) -> Option<Tag>{
        self.parameters.get("tag").map(|e| e.as_str().try_into().ok()).flatten()
    }

    pub fn get_name(&self) -> Option<String>{
        self.parameters.get("name").cloned()
    }
}


