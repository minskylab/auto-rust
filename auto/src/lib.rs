mod api;
mod cache;
mod data;
mod generator;

use std::fs::read_to_string;

use api::generic_chat_completion;
use cache::ResolvedSourceCode;
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
    dotenv().ok();

    println!("args: {:?}", args);

    // detect #[llm_tool(live)] arg

    let is_live = args.to_string().contains("live");

    let ast: ItemFn = syn::parse(input).expect("Failed to parse input as a function");

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

    // println!("{}", source_code_context);

    let system_message = format!("
    You are an advanced AI, trained on the most modern  architecture, with expertise in Rust programming. Your task is to generate the body of a Rust function based on its signature. Please adhere to these guidelines:
    
    1. Receive the Function Signature: The signature will be provided in a standard Rust format, e.g., 'fn calculate_pi_with_n_iterations(n: u64) -> f64'. Focus on understanding the function's name, parameters, and return type.
    2. Generate Only the Function Body: You are required to write Rust code that fulfills the requirements of the function signature. This code should be the function body only, without including the function signature or any other wrapping code.
    3. Exclude Non-Essential Content: Your response must strictly contain valid Rust code applicable within the function's curly braces. Do not include comments, attributes, nested functions, or any redundant repetitions of the function signature. Do not include any explanation or additional text outside of the function body.
    4. Maintain Simplicity and Clarity: Avoid external crates, unnecessary imports, or extra features like feature flags. Use standard Rust libraries and functionalities. The code should be clear, maintainable, and compile-ready.
    5. Adhere to Rust Best Practices: Ensure that the generated code is idiomatic, efficient, and adheres to Rust standards and best practices.
    
    Example:
    INPUT SIGNATURE: 'fn calculate_pi_with_n_iterations(n: u64) -> f64'
    EXPECTED OUTPUT (Function Body Only):
        let mut pi = 0.0;
        let mut sign = 1.0;
        for i in 0..n {{
            pi += sign / (2 * i + 1) as f64;
            sign = -sign;
        }}
        4.0 * pi
    
    Don't forget only respond with the function body. Don't include nature language text or explanation in your response.

    Global Context:
    {}
    ", source_code_context);

    let mut prompt_input = String::new();

    let fn_header = ast.sig.to_token_stream().to_string();

    for attr in ast.attrs {
        let data = attr.to_token_stream().to_string();

        prompt_input.push_str(&data);
        prompt_input.push('\n');
    }

    prompt_input.push_str(&fn_header);

    println!("prompt_input: {}", prompt_input);

    let hash = md5::compute(prompt_input.as_bytes());

    let hash_string = format!("{:x}", hash);

    let existing_resolution = ResolvedSourceCode::load(&hash_string);

    if !is_live {
        if let Some(resolution) = existing_resolution {
            return resolution.implementation.parse().unwrap();
        }
    }

    let res = generic_chat_completion(system_message, prompt_input.clone()).unwrap();

    println!("res: {:?}", res);

    let body_str = res
        .choices
        .first()
        .unwrap()
        .message
        .content
        .trim()
        .trim_matches('`')
        .trim_matches('\'')
        .to_string()
        .lines()
        .skip_while(|line| line.starts_with("rust") || line.starts_with("#["))
        .collect::<Vec<&str>>()
        .join("\n");

    let implementation = format!(
        "{} {{
            {}
        }}",
        prompt_input.clone(),
        body_str
    );

    println!("impl:\n {}", implementation);

    let new_resolution = ResolvedSourceCode {
        implementation: implementation.clone(),
        hash: format!("{:x}", hash),
        prompt_input,
    };

    new_resolution.save();

    implementation.parse().unwrap()
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
