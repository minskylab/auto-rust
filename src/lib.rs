mod api;
mod data;
mod generator;

use dotenv::dotenv;
use proc_macro::TokenStream;
use syn::{ItemFn, __private::ToTokens};

use crate::generator::generate_body_function_from_head;

#[proc_macro]
pub fn implement(_item: TokenStream) -> TokenStream {
    // TODO: Evaluate the use of dotenv in this crate
    dotenv().ok();

    let implemented_fn = generate_body_function_from_head(_item.to_string()).unwrap();

    println!("{}", implemented_fn);

    implemented_fn.parse().unwrap()
}

#[proc_macro_attribute]
pub fn auto_implement(args: TokenStream, input: TokenStream) -> TokenStream {
    let ast: ItemFn = syn::parse(input).expect("Failed to parse input as a function");

    // Search for the information within the attributes.

    let mut prompt_input = String::new();

    let fn_header = ast.sig.to_token_stream().to_string();

    println!("Function header: {}", fn_header);

    for attr in ast.attrs {
        let data = attr.to_token_stream().to_string();

        prompt_input.push_str(&data);
        prompt_input.push('\n');
    }

    prompt_input.push_str(&fn_header);

    println!("Information extracted: {:?}", prompt_input);

    dotenv().ok();

    let implemented_fn = generate_body_function_from_head(prompt_input).unwrap();

    println!("{}", implemented_fn);

    implemented_fn.parse().unwrap()
}
