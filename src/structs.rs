use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use regex::Regex;

/// This corresponds to a link between the LLVM local variable and the debug information
/// from a line such as:
/// `call void @llvm.dbg.value(metadata %"std::fmt::Formatter"* %f, metadata !214, metadata !DIExpression()), !dbg !217`
#[derive(Debug, Clone)]
pub struct LLVMToRustMetadataLink {
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
pub struct RustDebugMetadataExplanation {
    /// In the example above: `!1298`
    pub location_tag: String,
    /// In the example above: `!DIDerivedType`
    pub variant: String,
    /// A HashMap of each name:value, such as `tag` -> `DwTagPointerType`
    pub parameters: HashMap<String, String>,
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

impl RustDebugMetadataExplanation{
    pub fn get_variant(&self) -> Option<Variant>{
        self.variant.as_str().try_into().ok()
    }
    pub fn get_tag(&self) -> Option<Tag>{
        self.parameters.get("tag").map(|e| e.as_str().try_into().ok()).flatten()
    }
}


