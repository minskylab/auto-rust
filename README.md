# Auto Rust

<p align="center">
  <img width="640" src="public/auto_rust_example.png">
</p>

`auto-rust` is a Rust procedural macro that harnesses the power of Large Language Models (LLMs) to generate code at compile time.  Write a function signature, add a doc comment describing the desired functionality, and let `auto-rust` fill in the implementation details!

## ⚠️ Warning

Auto-Rust is currently under development and is not yet suitable for production use. While you are welcome to try it out and provide feedback, we caution that it may have an incomplete implementation and may not function as intended. 

## Features

* **AI-Powered Code Generation:**  Leverage the capabilities of LLMs to generate Rust function bodies based on your specifications.
* **Compile-Time Integration:** Seamlessly integrate code generation into your Rust projects with our easy-to-use procedural macro.
* **Context-Aware Generation:** Auto-Rust considers your function signature, doc comments, and some code within your project to generate contextually relevant implementations.

## Getting Started

**1. Installation**

Add `auto-rust` as a dependency in your `Cargo.toml` file:

```toml
[dependencies]
auto-rust = "0.1.4"
```

**2. API Key**

Auto-Rust requires an OpenAI API key. Create a `.env` file in the root of your project and add your key:

```bash
OPENAI_API_KEY=<your-openai-api-key>
```

**3. Usage**

Annotate your function with the `#[llm_tool]` attribute and provide a clear doc comment explaining the desired behavior:

```rust
use auto_rust::llm_tool;

#[llm_tool]
#[handler]
/// The response will be an HTML layout with the name and the number of letters in the name.
/// Ensure to return a content type of text/html.
fn hello(Path(name): Path<String>) -> impl IntoResponse {
    todo!()
}
```
**Live Reload**
To enable live reload for seamless development, use the `live` argument for the `llm_tool` macro. 
```rust
#[llm_tool(live)] 
```
Be aware that enabling live reload will call the LLM on each compilation, potentially impacting compile times.
## Example

```rust
use auto_rust::llm_tool;
use poem::{
    get, handler, listener::TcpListener, middleware::Tracing, web::Path, EndpointExt, IntoResponse,
    Route, Server,
};

#[llm_tool]
#[handler]
/// The response will be an html layout with the name and the number of letters in the name.
/// Ensure to return a content type of text/html.
fn hello(Path(name): Path<String>) -> impl IntoResponse {
    todo!()
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // ...
}
```

**Generated Code:**

```rust
#[handler]
/// The response will be an html layout with the name and the number of letters in the name.
/// Ensure to return a content type of text/html.
fn hello(Path(name): Path<String>) -> impl IntoResponse {
    let response_html = format!(
    r#"
<!DOCTYPE html>
<html>
<head>
    <title>Hello, {name}!</title>
</head>
<body>
    <h1>Hello, {name}!</h1>
    <p>The length of your name is {len} letters.</p>
</body>
</html>
"#,
    name = name,
    len = name.len()
);

poem::Response::builder()
    .header("Content-Type", "text/html; charset=utf-8")
    .body(response_html)
}
```

## Limitations

* **LLM Non-Determinism:** LLM output can vary. You might need to experiment and iterate to achieve the desired results.
* **Limited Context:** Currently, Auto-Rust's context is primarily based on your function signature and doc comments. It has limited awareness of the broader project structure.
* **Dependency Management:** Auto-Rust does not automatically add `use` statements for newly introduced dependencies in the generated code.

## Contributing

We welcome contributions from the Rust community! If you encounter issues, have suggestions, or want to contribute to the development of `auto-rust`, please feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT and Apache-2.0 licenses. 
