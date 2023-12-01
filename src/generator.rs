use std::error::Error;

use crate::api::open_ai_chat_completions;

pub fn generate_body_function_from_head(
    head: String,
    extra_context: Option<String>,
) -> Result<String, Box<dyn Error>> {
    let system_message = "
    You are an advanced AI trained on the GPT-4 architecture, specializing in Rust programming. Your primary role is to generate Rust function bodies based on provided function signatures. Here's how you'll approach each task:
    
    1. Understand the Function Signature: Analyze the provided function signature to determine the function's purpose and expected behavior.
    2. Plan the Implementation: Conceptualize the necessary steps and logic required to implement the function.
    3. Write the Code: Generate the Rust code for the function body that fulfills the requirements of the function signature.
    4. Ensure Clarity and Efficiency: Write code that is clear, concise, and efficient, avoiding unnecessary complexity.
    5. Compliance with Constraints: Do not include triple backticks, the original function signature, or extraneous explanations in your response. Stick to plain Rust code for the function body.
    
    Respond with only the function body as plain Rust code. Each response must be a direct implementation of the given function signature, tailored to its specific requirements.
    
    Example 1:
    INPUT: /implement fn my_ip() -> String
    OUTPUT:
        use std::net::UdpSocket;
        let udp_socket = UdpSocket::bind(\"0.0.0.0:0\").unwrap();
        udp_socket.connect(\"8.8.8.8:80\").unwrap();
        let socket_addr = udp_socket.local_addr().unwrap();
        let ip_addr = socket_addr.ip();
        ip_addr.to_string()
    
    Example 2:
    INPUT: /implement fn hello_world() -> String
    OUTPUT:
        \"Hello World\".to_string()
    
    Example 3:
    INPUT: /implement fn hello_world(name: String) -> String
    OUTPUT:
        format!(\"Hello {}!\", name)
    ".to_string();

    let user_message = extra_context
        .map(|c| format!("Extra context: {}\n", c))
        .unwrap_or("".to_string())
        + &format!("/implement {}", head);

    // println!("User message: {}", user_message);

    let res = open_ai_chat_completions(system_message, user_message).unwrap();

    let body_str = res.choices.first().unwrap().to_owned().message.content;

    //TODO: improve the prompt to eliminate the need for this
    let body_str = body_str.trim_matches('`').to_string();

    let implementation = format!(
        "{} {{
            {}
        }}",
        head, body_str
    );

    Ok(implementation)
}
