mod api;
mod data;
mod generator;

use dotenv::dotenv;

use proc_macro::TokenStream;
use syn::{ItemFn, __private::ToTokens};

use crate::generator::{generate_body_function_from_head, minimal_llm_function};

/// This macro gets an input like "String, "This is a llm generated function"" and returns a function that returns a String
#[proc_macro]
pub fn auto_generate(item: TokenStream) -> TokenStream {
    dotenv().ok();

    let res = minimal_llm_function(item.to_string());
    // println!("{:?}", res);

    res.parse().unwrap()
}

#[proc_macro_attribute]
pub fn auto_implement(args: TokenStream, input: TokenStream) -> TokenStream {
    let ast: ItemFn = syn::parse(input).expect("Failed to parse input as a function");

    let context = args.to_string();

    let mut prompt_input = String::new();

    let fn_header = ast.sig.to_token_stream().to_string();

    for attr in ast.attrs {
        let data = attr.to_token_stream().to_string();

        prompt_input.push_str(&data);
        prompt_input.push('\n');
    }

    prompt_input.push_str(&fn_header);

    dotenv().ok();

    let implemented_fn = generate_body_function_from_head(prompt_input, Some(context)).unwrap();

    implemented_fn.parse().unwrap()
}
