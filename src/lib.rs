mod api;
mod data;
mod generator;

use std::fs::read_to_string;

use dotenv::dotenv;

use ignore::Walk;
use proc_macro::TokenStream;
// use quote::quote;

use syn::{ItemFn, __private::ToTokens};

// use git2::Repository;

use crate::generator::{generate_body_function_from_head, minimal_llm_function};

/// This macro gets an input like "String, "This is a llm generated function"" and returns a function that returns a String
#[proc_macro]
pub fn auto_generate(item: TokenStream) -> TokenStream {
    dotenv().ok();

    let res = minimal_llm_function(item.to_string());
    // println!("{:?}", res);

    res.parse().unwrap()
}

struct RawSourceCode {
    path: String,
    language: String,
    content: String,
}

#[proc_macro_attribute]
pub fn llm_tool(args: TokenStream, input: TokenStream) -> TokenStream {
    let cargo_toml_path = std::env::var("CARGO_MANIFEST_DIR").unwrap_or("".to_string());

    println!("{:?}", cargo_toml_path);

    let mut source_code = vec![];

    for result in Walk::new(cargo_toml_path) {
        match result {
            Ok(entry) => {
                if entry.path().is_file() {
                    if let Ok(Some(kind)) = hyperpolyglot::detect(entry.path()) {
                        let path = format!("{}", entry.path().display());

                        let content = read_to_string(path.clone()).unwrap();
                        if content.lines().count() > 500 {
                            continue;
                        }

                        println!("{}: {:?}", path, kind);

                        let language = kind.language().to_string();

                        source_code.push(RawSourceCode {
                            path,
                            content,
                            language,
                        });
                    }
                }
            }
            Err(err) => println!("ERROR: {}", err),
        }
    }

    let source_code_context = source_code
        .iter()
        .map(|x| {
            format!(
                "## {}\n```{}\n{}\n```\n",
                x.path,
                x.language.to_lowercase(),
                x.content
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    println!("{}", source_code_context);
    input
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
