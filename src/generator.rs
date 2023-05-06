use std::error::Error;

use proc_macro::TokenStream;

use crate::api::open_ai_chat_completions;

pub fn generate_body_function_from_head(head: TokenStream) -> Result<String, Box<dyn Error>> {
    let system_message = "You are an AI code assistant trained on the GPT-4 architecture. Your task is to generate Rust function body implementations based only on the provided function signatures. When the user provides a function signature using the command '/complete', your response must be the plain text function body, without any explanations, formatting, or code blocks. Do not include the function signature, function name, or any other information in your response. Triple backticks (```) and function signatures are strictly prohibited in your response. Responding with any prohibited content will result in a penalty. 
    example 1:
    INPUT: /complete fn my_ip() -> String
    OUTPUT: 
        use std::net::UdpSocket;
    
        let udp_socket = UdpSocket::bind(\"0.0.0.0:0\").unwrap();
        udp_socket.connect(\"8.8.8.8:80\").unwrap();
        let socket_addr = udp_socket.local_addr().unwrap();
        let ip_addr = socket_addr.ip();
        ip_addr.to_string() 
    example 2:
    INPUT: /complete fn hello_world() -> String
    OUTPUT: \"Hello World\".to_string()
    example 3:
    INPUT: /complete fn hello_world(name: String) -> String
    OUTPUT: format!(\"Hello {}!\", name)
".to_string();

    let user_message = format!("/complete {}", head);

    let res = open_ai_chat_completions(system_message, user_message).unwrap();

    let body_str = res.choices.first().unwrap().to_owned().message.content;

    //TODO: improve the prompt to eliminate the need for this
    let body_str = body_str.trim_matches('`').to_string();

    let implementation = format!("{} {{{}}}", head, body_str);

    Ok(implementation)
}
