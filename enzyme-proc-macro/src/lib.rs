use proc_macro::TokenStream as TokenStreamBuiltin;
use serde::Serialize;
use std::io::Write;
use syn::export::TokenStream;
use syn::{parse_macro_input, FnArg, ForeignItemFn, ItemFn, Pat, Receiver};

const OUT_DIR: &str = "diff";

#[derive(Clone, Debug, Serialize)]
struct Parameters {
    name: String,
    mutable: bool,
}

#[derive(Clone, Debug, Serialize)]
struct FunctionToDiff {
    name: String,
    parameters: Vec<Parameters>,
}

#[derive(Clone, Debug, Serialize)]
struct DiffResultFunction {
    diff_result_fn_name: String,
    original_fn_name: String,
    parameters: Vec<Parameters>,
}

#[proc_macro_attribute]
pub fn autodiff(
    attribute_args: TokenStreamBuiltin,
    input: TokenStreamBuiltin,
) -> TokenStreamBuiltin {
    let input_clone = input.clone();
    let parsed_fn = parse_macro_input!(input_clone as ItemFn);
    let function_name = parsed_fn.sig.ident.to_string();
    let function_inputs = parsed_fn.sig.inputs;
    let mut parameters = vec![];
    for inputs in &function_inputs {
        match inputs {
            FnArg::Receiver(_) => {
                panic!("Can only use autodiff in free functions, no self can be used yet.")
            }
            FnArg::Typed(ty) => {
                let types = &*ty.pat;
                match types {
                    Pat::Ident(iden) => {
                        let is_mutable = iden.mutability.is_some();
                        let parameter_name = iden.ident.to_string();
                        parameters.push(Parameters {
                            name: parameter_name,
                            mutable: is_mutable,
                        });
                    }
                    _ => {
                        panic!("Can only diff plain function with only identifier as args");
                    }
                }
            }
        }
    }
    let f = FunctionToDiff {
        name: function_name,
        parameters,
    };
    std::fs::create_dir_all(OUT_DIR).unwrap();
    let mut to_auto_diff = std::fs::File::create(format!("{}/{}.json", OUT_DIR, f.name)).unwrap();
    let as_str = serde_json::to_string_pretty(&f).unwrap();
    to_auto_diff.write_all(as_str.as_bytes()).unwrap();
    input
}

#[proc_macro_attribute]
pub fn autodiff_result(
    attribute_args: TokenStreamBuiltin,
    input: TokenStreamBuiltin,
) -> TokenStreamBuiltin {
    let original_fun_name = attribute_args.to_string();
    if original_fun_name.is_empty() {
        panic!("Need to specify function to diff")
    }
    let input_clone = input.clone();
    let parsed_fn = parse_macro_input!(input_clone as ForeignItemFn);
    let function_inputs = parsed_fn.sig.inputs;
    let mut parameters = vec![];
    for inputs in &function_inputs {
        match inputs {
            FnArg::Receiver(_) => {
                panic!("Can only use autodiff in free functions, no self can be used yet.")
            }
            FnArg::Typed(ty) => {
                let types = &*ty.pat;
                match types {
                    Pat::Ident(iden) => {
                        let is_mutable = iden.mutability.is_some();
                        let parameter_name = iden.ident.to_string();
                        parameters.push(Parameters {
                            name: parameter_name,
                            mutable: is_mutable,
                        });
                    }
                    _ => {
                        panic!("Can only diff plain function with only identifier as args");
                    }
                }
            }
        }
    }
    let f = DiffResultFunction {
        diff_result_fn_name: parsed_fn.sig.ident.to_string(),
        original_fn_name: original_fun_name.clone(),
        parameters,
    };
    std::fs::create_dir_all(OUT_DIR).unwrap();
    let mut auto_diff_res =
        std::fs::File::create(format!("{}/{}.json", OUT_DIR, f.diff_result_fn_name)).unwrap();
    let as_str = serde_json::to_string_pretty(&f).unwrap();
    auto_diff_res.write_all(as_str.as_bytes()).unwrap();
    input
}
