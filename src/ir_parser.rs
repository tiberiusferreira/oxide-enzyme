use std::io::Read;
use regex::Regex;
use std::collections::HashMap;


#[derive(Debug, Clone)]
struct DebugMetadata{
    local_var_type: String,
    local_var_name: String,
    location_tag: String,
}

#[derive(Debug, Clone)]
struct DebugMetadataExplanation{
    location_tag: String,
    variant: String,
    parameters: HashMap<String, String>,
}

fn get_all_params(input: String) -> HashMap<String, String>{
    let debug_value_description = r#"([^:\s]+):\s([^,]+)[,\s]?"#;
    let re = Regex::new(debug_value_description).unwrap();
    let mut debug_metadatas: HashMap<String, String> = HashMap::new();

    for caps in re.captures_iter(&input){
        debug_metadatas.insert(caps[1].to_string(), caps[2].to_string());
    }
    debug_metadatas
}

pub fn main(){
    let mut string = String::new();
    let mut file = std::fs::File::open("oxide_enzyme_replaced.ll").unwrap();
    file.read_to_string(&mut string).unwrap();
    let debug_value_regex = r#"@llvm\.dbg\.value\(metadata (\S+) (\S+), metadata ([^,]+), metadata !DIExpression\(\)\)"#;
    let re = Regex::new(debug_value_regex).unwrap();
    // call void @llvm.dbg.declare(metadata %Tensor* %left, metadata !1323, metadata !DIExpression()), !dbg !1344
    // println!("{}", re.is_match("@llvm.dbg.declare(metadata %Tensor* %left, metadata !1323, metadata !DIExpression())"));
    let mut debug_metadatas = vec![];
    for caps in re.captures_iter(&string){
        assert_eq!(caps.len(), 4);
        debug_metadatas.push(DebugMetadata{
            local_var_type: caps[1].to_string(),
            local_var_name: caps[2].to_string(),
            location_tag: caps[3].to_string()
        });
    }
    let debug_value_description = r#"(!\d+) = (!\S+)\((.+)\)"#;
    let re = Regex::new(debug_value_description).unwrap();
    let mut debug_metadatas_explanation = vec![];
    for caps in re.captures_iter(&string){
        assert_eq!(caps.len(), 4);
        debug_metadatas_explanation.push(DebugMetadataExplanation{
            location_tag: caps[1].to_string(),
            variant: caps[2].to_string(),
            parameters: get_all_params(caps[3].to_string())
        });
    }
    let single = debug_metadatas[0].clone();
    println!("{:#?}", single);
    let mut tag_loc = single.clone().location_tag;
    loop {
        let found = debug_metadatas_explanation
            .iter()
            .find(|a| a.location_tag == tag_loc);
        match found{
            None => {
                break;
            }
            Some(found) => {
                println!("{:#?}", found);
                if let Some(ty) = found.parameters.get("type"){
                    tag_loc = ty.clone();
                }else if let Some(ty) = found.parameters.get("baseType"){
                        tag_loc = ty.clone();
                }else{
                    break;
                }
            }
        }


    }
    // println!("{:#?}", debug_metadatas);
}
