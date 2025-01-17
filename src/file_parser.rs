use crate::structs::{LLVMLocalTypeVariableDebugInfo, LLVMDebugTypeInformation};
use regex::Regex;
use std::io::Read;
use crate::{LLVMIRMetadata, extract_llvm_multiple_tags, get_all_params};
use std::collections::HashMap;

impl LLVMIRMetadata{
    pub fn new(file_path: &str) -> Self{
        let mut file_as_string = String::new();
        let mut file = std::fs::File::open(file_path).unwrap();
        file.read_to_string(&mut file_as_string).unwrap();
        let llvm_to_rust_metadata_link = extract_llvm_to_rust_metadata(&file_as_string);
        let rust_metadata = extract_rust_metadata(&file_as_string);
        let multi_tags_tag = extract_llvm_multiple_tags_tag(&file_as_string);
        Self{
            multiple_tags_tag: multi_tags_tag,
            llvm_local_type_variable_debug_info: llvm_to_rust_metadata_link,
            llvm_debug_type_information: rust_metadata
        }
    }
}


pub fn extract_llvm_multiple_tags_tag(file_as_string: &str) -> HashMap<String, Vec<String>>{
    // works on lines like: @llvm.dbg.value(metadata %"std::fmt::Formatter"* %f, metadata !214, metadata !DIExpression()), !dbg !217
    let line_number_and_tags = r#"(!\d+) = !\{(!.+)?\}"#;
    let regex = Regex::new(line_number_and_tags).unwrap();
    let mut multiple_tags_tag_vec = HashMap::new();
    for caps in regex.captures_iter(&file_as_string){
        assert_eq!(caps.len(), 3); // don't trust this, capture group could have no group
        let location_tag = &caps[1];
        let tags = if let Some(tags) = &caps.get(2){
            extract_llvm_multiple_tags(tags.as_str())
        }else{
            vec![]
        };
        multiple_tags_tag_vec.insert(location_tag.to_string(), tags);
    };
    multiple_tags_tag_vec
}

/// Parses lines like: @llvm.dbg.value(metadata %"std::fmt::Formatter"* %f, metadata !214, metadata !DIExpression()), !dbg !217
pub fn extract_llvm_to_rust_metadata(file_as_string: &str) -> Vec<LLVMLocalTypeVariableDebugInfo>{
    let debug_value_regex = r#"@llvm\.dbg\.value\(metadata (\S+) (\S+), metadata ([^,]+), metadata !DIExpression\(\)\)"#;
    let regex = Regex::new(debug_value_regex).unwrap();
    let mut debug_metadatas = vec![];
    for caps in regex.captures_iter(&file_as_string){
        assert_eq!(caps.len(), 4);
        debug_metadatas.push(LLVMLocalTypeVariableDebugInfo {
            // the first capture contains the whole regex match
            local_var_type: caps[1].to_string(),
            local_var_name: caps[2].to_string(),
            location_tag: caps[3].to_string()
        });
    };
    debug_metadatas
}

pub fn extract_rust_metadata(file_as_string: &str) -> Vec<LLVMDebugTypeInformation>{
    let debug_value_description = r#"(!\d+) = (!\S+)\((.+)\)"#;
    let re = Regex::new(debug_value_description).unwrap();
    let mut debug_metadatas_explanation = vec![];
    for caps in re.captures_iter(&file_as_string){
        assert_eq!(caps.len(), 4);
        debug_metadatas_explanation.push(LLVMDebugTypeInformation {
            location_tag: caps[1].to_string(),
            variant: caps[2].to_string(),
            parameters: get_all_params(caps[3].to_string())
        });
    }
    debug_metadatas_explanation
}


