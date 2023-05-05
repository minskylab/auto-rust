use reqwest::header;
use serde_json::json;
use std::env;

use crate::data::ChatCompletionResponse;

pub fn open_ai_chat_completions(
    system_message: String,
    user_message: String,
) -> Result<ChatCompletionResponse, Box<dyn std::error::Error>> {
    let open_ai_key = env::var("OPENAI_API_KEY").unwrap_or("".to_string());

    let mut headers = header::HeaderMap::new();

    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert(
        "Authorization",
        format!("Bearer {}", open_ai_key).parse().unwrap(),
    );

    let client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let res = client
        .post("https://api.openai.com/v1/chat/completions")
        .headers(headers)
        .body(
            json!({
                "model": "gpt-3.5-turbo",
                "messages": [
                    {"role": "system", "content": system_message},
                    {"role": "user", "content": user_message}
                ]
            })
            .to_string(),
        )
        .send()?
        .json::<ChatCompletionResponse>()?;

    // println!("{:?}", res);

    Ok(res)
}
