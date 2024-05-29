use std::error::Error;

use crate::api::open_ai_chat_completions;

pub fn generate_body_function_from_head(
    head: String,
    extra_context: Option<String>,
) -> Result<String, Box<dyn Error>> {
    let system_message = "
    You are an advanced AI, trained on the GPT-4 architecture, with expertise in Rust programming. Your task is to generate the body of a Rust function based on its signature. Please adhere to these guidelines:
    
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
        for i in 0..n {
            pi += sign / (2 * i + 1) as f64;
            sign = -sign;
        }
        4.0 * pi
    ".to_string();

    let user_message = extra_context
        .map(|c| format!("Extra context: {}\n", c))
        .unwrap_or("".to_string())
        + &format!(
            "Function signature: {}\nGenerate only the function body.",
            head
        );

    // Call the external LLM API to get the function body
    let res = open_ai_chat_completions(system_message, user_message)?;

    let body_str = res.choices.first().unwrap().message.content.trim();

    // Strip backticks and clean up the response
    let body_str = body_str.trim_matches('`').to_string();

    // Ensure the response contains only the function body
    let body_str = body_str
        .lines()
        .skip_while(|line| line.starts_with("rust") || line.starts_with("#["))
        .collect::<Vec<&str>>()
        .join("\n");

    let implementation = format!(
        "{} {{
            {}
        }}",
        head, body_str
    );

    println!("implementation: {}", implementation);

    Ok(implementation)
}

pub fn minimal_llm_function(input: String) -> String {
    let system_message = "Your task is respond with a string with double quotes.".to_string();

    let res = open_ai_chat_completions(system_message, input).unwrap();

    res.choices.first().unwrap().to_owned().message.content
}
