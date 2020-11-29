use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};

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
struct DILocalVariable{
    file: String,
    name: Option<String>,
    arg: Option<String>,
    line: String,
    r#type: String,
    scope: String,
}

#[derive(Clone, Debug)]
struct DIDerivedType{
    tag: String,
    align: String,
    dwarf_address_space: Option<String>,
    size: Option<String>,
    name: Option<String>,
    base_type: String,
}

#[derive(Clone, Debug)]
struct DIBasicType{
    encoding: String,
    size: Option<String>,
    name: String,
}

#[derive(Clone, Debug)]
struct DICompositeType{
    tag: String,
    identifier: Option<String>,
    elements: String,
    vtable_holder: Option<String>,
    flags: Option<String>,
    name: Option<String>,
    file: Option<String>,
    align: String,
}

#[derive(Clone, Debug)]
struct DITemplateTypeParameter{
    name: String,
    r#type: String,
}

#[derive(Debug, Clone)]
pub enum TypeAST{
    DILocalVariable(DILocalVariable),
    DerivedType(DIDerivedType),
    DIBasicType(DIBasicType),
    DICompositeType(DICompositeType),
    DITemplateTypeParameter(DITemplateTypeParameter)
}

#[derive(Debug, Clone)]
pub struct Ast{
    pub inner: HashMap<String, TypeAST>
}

pub fn parse_llvm_debug_type_information(debug_type_info: &LLVMDebugTypeInformation) -> TypeAST{
    return match debug_type_info.get_variant().expect("No Variant"){
        Variant::LocalVariable => {
        let get_val = |field: &str| debug_type_info.parameters.get(field).expect(&format!("No {} in DILocalVariable", field)).clone();
        let get_val_optional = |field: &str| debug_type_info.parameters.get(field).cloned();
            TypeAST::DILocalVariable(DILocalVariable{
                file: get_val("file"),
                name: get_val_optional("name"),
                arg: get_val_optional("arg"),
                line: get_val("line"),
                r#type: get_val("type"),
                scope: get_val("scope")
            })
        }
        Variant::SubroutineType => {unimplemented!()}
        Variant::DerivedType => {
            let get_val = |field: &str| debug_type_info.parameters.get(field).expect(&format!("No {} in DIDerivedType", field)).clone();
            let get_val_optional = |field: &str| debug_type_info.parameters.get(field).cloned();
            TypeAST::DerivedType(DIDerivedType{
                tag: get_val("tag"),
                align: get_val("align"),
                dwarf_address_space: get_val_optional("dwarfAddressSpace"),
                size: get_val_optional("size"),
                name: get_val_optional("name"),
                base_type: get_val("baseType")
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
            TypeAST::DICompositeType(DICompositeType{
                tag: get_val("tag"),
                identifier: get_val_optional("identifier"),
                elements: get_val("elements"),
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
        Variant::Enumerator => {unimplemented!()}
        Variant::DISubrange => {unimplemented!()}
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
    SubroutineType,
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
            "!DISubroutineType" => Ok(Self::SubroutineType),
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


