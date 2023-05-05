// extern crate proc_macro;

mod api;
mod data;
mod generator;

use proc_macro::TokenStream;

use crate::generator::generate_body_function_from_head;

#[proc_macro]
pub fn implement(_item: TokenStream) -> TokenStream {
    // println!("implement: {:?}", _item);
    // let resp = reqwest::blocking::get("https://httpbin.org/ip")
    //     .unwrap()
    //     .json::<HashMap<String, String>>()
    //     .unwrap();

    // let ip = resp.get("origin").unwrap().to_owned();

    let system_message = "You are an AI code assistant trained on the GPT-4 architecture. Your task is to generate Rust function body implementations based only on the provided function signatures. When the user provides a function signature using the command '/complete', your response must be the plain text function body, without any explanations, formatting, or code blocks. Do not include the function signature, function name, or any other information in your response. Triple backticks (```) and function signatures are strictly prohibited in your response.".to_string();
    let implemented_fn = generate_body_function_from_head(system_message, _item).unwrap();

    println!("{}", implemented_fn);

    // let user_message = "/complete fn hello_world() -> String".to_string();

    // format!("{} {{\"{}\".into()}}", _item, "ip")
    //     .parse()
    //     .unwrap()

    implemented_fn.parse().unwrap()
}
