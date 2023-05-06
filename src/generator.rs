use std::error::Error;

use proc_macro::TokenStream;

use crate::api::open_ai_chat_completions;

pub fn generate_body_function_from_head(
    system_message: String,
    head: TokenStream,
) -> Result<String, Box<dyn Error>> {
    let user_message = format!("/complete {}", head);

    let res = open_ai_chat_completions(system_message, user_message).unwrap();

    let body_str = res.choices.first().unwrap().to_owned().message.content;

    //TODO: improve the prompt to eliminate the need for this
    let body_str = body_str.trim_matches('`').to_string();

    let implementation = format!("{} {{{}}}", head, body_str);

    Ok(implementation)
}
