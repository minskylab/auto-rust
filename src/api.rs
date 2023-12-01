use reqwest::header;
use serde_json::json;
use std::env;

use crate::data::ChatCompletionResponse;

pub fn open_ai_chat_completions(
    system_message: String,
    user_message: String,
) -> Result<ChatCompletionResponse, Box<dyn std::error::Error>> {
    let open_ai_key = env::var("OPENAI_API_KEY").unwrap();
    let model_name = env::var("OPENAI_MODEL_NAME").unwrap_or("gpt-3.5-turbo".to_string());

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

    let body = json!({
        "model": model_name,
        "messages": [
            {"role": "system", "content": system_message},
            {"role": "user", "content": user_message}
        ]
    });

    let res = client
        .post("https://api.openai.com/v1/chat/completions")
        .headers(headers)
        .body(body.to_string())
        .send()?
        .json::<ChatCompletionResponse>()?;

    Ok(res)
}
