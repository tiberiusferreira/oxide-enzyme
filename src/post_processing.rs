use std::io::{Read, Write};

const ENZYME_CONST_PLACEHOLDER: &str = "i128 1321523312";
const ENZYME_CONST: &str = "metadata !\"enzyme_const\"";

const ENZYME_DUP_PLACEHOLDER: &str = "i128 314210384213";
const ENZYME_DUP: &str = "metadata !\"enzyme_dup\"";

fn main(){
    let mut string = String::new();
    let mut file = std::fs::File::open("oxide_enzyme.ll").unwrap();
    file.read_to_string(&mut string).unwrap();
    let replaced = string.replace(ENZYME_CONST_PLACEHOLDER, ENZYME_CONST)
        .replace(ENZYME_DUP_PLACEHOLDER, ENZYME_DUP);
    std::fs::File::create("oxide_enzyme_replaced.ll").unwrap().write_all(replaced.as_bytes()).unwrap();
}