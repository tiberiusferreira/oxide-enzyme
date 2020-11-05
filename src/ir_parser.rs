use std::io::Read;
use regex::Regex;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};

/// This corresponds to a link between the LLVM local variable and the debug information
/// from a line such as:
/// `call void @llvm.dbg.value(metadata %"std::fmt::Formatter"* %f, metadata !214, metadata !DIExpression()), !dbg !217`
#[derive(Debug, Clone)]
struct LLVMToRustDebugMetadataLink {
    /// In the example above: `%Tensor`
    local_var_type: String,
    /// In the example above: `%left`
    local_var_name: String,
    /// In the example above: `!1323`
    location_tag: String,
}

/// This corresponds to the Rust Metadata information, such as
/// !1298 = !DIDerivedType(tag: DwTagPointerType, name: "&mut alloc::vec::Vec<f64>", baseType: !6, size: 64, align: 64, dwarfAddressSpace: 0)
#[derive(Debug, Clone)]
struct RustDebugMetadataExplanation {
    /// In the example above: `!1298`
    location_tag: String,
    /// In the example above: `!DIDerivedType`
    variant: String,
    /// A HashMap of each name:value, such as `tag` -> `DwTagPointerType`
    parameters: HashMap<String, String>,
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


fn extract_llvm_to_rust_metadata(file_as_string: &str) -> Vec<LLVMToRustDebugMetadataLink>{
    // works on lines like: @llvm.dbg.value(metadata %"std::fmt::Formatter"* %f, metadata !214, metadata !DIExpression()), !dbg !217
    let debug_value_regex = r#"@llvm\.dbg\.value\(metadata (\S+) (\S+), metadata ([^,]+), metadata !DIExpression\(\)\)"#;
    let regex = Regex::new(debug_value_regex).unwrap();
    let mut debug_metadatas = vec![];
    for caps in regex.captures_iter(&file_as_string){
        assert_eq!(caps.len(), 4);
        debug_metadatas.push(LLVMToRustDebugMetadataLink {
            // the first capture contains the whole regex match
            local_var_type: caps[1].to_string(),
            local_var_name: caps[2].to_string(),
            location_tag: caps[3].to_string()
        });
    };
    debug_metadatas
}

fn extract_rust_metadata(file_as_string: &str) -> Vec<RustDebugMetadataExplanation>{
    let debug_value_description = r#"(!\d+) = (!\S+)\((.+)\)"#;
    let re = Regex::new(debug_value_description).unwrap();
    let mut debug_metadatas_explanation = vec![];
    for caps in re.captures_iter(&file_as_string){
        assert_eq!(caps.len(), 4);
        debug_metadatas_explanation.push(RustDebugMetadataExplanation {
            location_tag: caps[1].to_string(),
            variant: caps[2].to_string(),
            parameters: get_all_params(caps[3].to_string())
        });
    }
    debug_metadatas_explanation
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

fn get_rust_debug_metadata(tag: &str, metadata_vec: &[RustDebugMetadataExplanation]) -> RustDebugMetadataExplanation{
    metadata_vec
        .iter()
        .find(|a| a.location_tag == tag).expect("Rust Metadata Tag not found!").clone()
}


fn describe_local_var(value: &RustDebugMetadataExplanation, rust_metadata: &[RustDebugMetadataExplanation]){
    println!("Getting local var type");
    let variable_type = get_rust_debug_metadata(&value.parameters.get("type").unwrap(), rust_metadata);

    if let Some(tag) = variable_type.get_tag(){
        if matches!(tag, Tag::TagPointerType){
            println!("Pointer!");
            return;
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
                                get_rust_debug_metadata(elements, rust_metadata);
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
        }
    }
    println!("{:?}", variable_type);
    panic!("Local variable type has not tag!");
}

pub fn main(){
    let mut file_as_string = String::new();
    let mut file = std::fs::File::open("oxide_enzyme_replaced.ll").unwrap();
    file.read_to_string(&mut file_as_string).unwrap();
    let linking_data = extract_llvm_to_rust_metadata(&file_as_string);
    let rust_metadata = extract_rust_metadata(&file_as_string);


    for local_llvm_type in linking_data{
        println!("Investigating: {:#?}", local_llvm_type);

        let single_rust_metadata = get_rust_debug_metadata(&local_llvm_type.location_tag, &rust_metadata);
        if let Some(variant) = single_rust_metadata.get_variant(){
            if matches!(variant, Variant::LocalVariable){
                println!("Is Local Variable");
                describe_local_var(&single_rust_metadata, &rust_metadata);
            }
        }
    }


    // println!("{:?}", get_rust_debug_metadata(&single_data.location_tag, &rust_metadata));

    // get_number_fields(&single_data, &rust_metadata);
    // let single = debug_metadatas[0].clone();
    // println!("{:#?}", single);
    // let mut tag_loc = single.clone().location_tag;
    // loop {
    //     let found = debug_metadatas_explanation
    //         .iter()
    //         .find(|a| a.location_tag == tag_loc);
    //     match found{
    //         None => {
    //             break;
    //         }
    //         Some(found) => {
    //             println!("{:#?}", found);
    //             if let Some(ty) = found.parameters.get("type"){
    //                 tag_loc = ty.clone();
    //             }else if let Some(ty) = found.parameters.get("baseType"){
    //                     tag_loc = ty.clone();
    //             }else{
    //                 break;
    //             }
    //         }
    //     }
    //}


    // println!("{:#?}", debug_metadatas);
}
