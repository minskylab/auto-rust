// extern crate proc_macro;

mod api;
mod data;
mod generator;

use dotenv::dotenv;
use proc_macro::TokenStream;
use syn::{parse, ItemFn, Lit, Meta, MetaNameValue, __private::ToTokens};
use syn::{Attribute, LitStr};

use crate::generator::generate_body_function_from_head;

#[proc_macro]
pub fn implement(_item: TokenStream) -> TokenStream {
    // TODO: Evaluate the use of dotenv in this crate
    dotenv().ok();

    let implemented_fn = generate_body_function_from_head(_item).unwrap();

    println!("{}", implemented_fn);

    implemented_fn.parse().unwrap()
}

#[proc_macro_attribute]
pub fn auto_implement(args: TokenStream, input: TokenStream) -> TokenStream {
    println!("{:?}", input);

    let ast: ItemFn = syn::parse(input.clone()).expect("Failed to parse input as a function");

    // Search for the information within the attributes.

    let mut target_info = String::new();

    let fn_header = ast.sig.to_token_stream().to_string();

    println!("Function header: {}", fn_header);

    for attr in ast.attrs {
        let data = attr.to_token_stream().to_string();
        // if attr.path().is_ident("doc") {
        // if let Ok(Meta::NameValue(meta_name_value)) = attr.parse_args() {
        //     let info = meta_name_value.value.to_token_stream().to_string();
        //     // if info.contains("This function calculates") {

        //     target_info = info;
        //     break;
        //     // }
        // }
        println!("{}", data)
        // }
    }

    println!("Information extracted: {:?}", target_info);

    input
}
