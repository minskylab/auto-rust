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
* **Built-in Caching:** Auto-Rust caches successful code generation results to accelerate your development workflow. Avoid unnecessary LLM calls and reduce compile times!

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

**Live Reload with `live` Argument**

To enable live reload for seamless development, use the `live` argument for the `llm_tool` macro. 

```rust
#[llm_tool(live)] 
```

Enabling `live` reload forces Auto-Rust to bypass its caching mechanism. On each compilation, it will send a request to the LLM, ensuring you always receive the most up-to-date code generation based on your function signature, doc comments, and current codebase context.  

**How Caching Works**

Auto-Rust implements a simple caching mechanism. Each time it generates code for a function:
   - It creates a hash based on the function signature, doc comments, and relevant code context.
   - This hash is used to store the LLM's generated code.
   - On subsequent compilations, Auto-Rust will first check if a cached result exists for the given hash. If found, it directly uses the cached code, preventing unnecessary LLM calls. 

Use the `live` argument when you want to prioritize receiving the most current code generation from the LLM, even if it leads to slightly increased compile times.

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

## Explanation for Developers

Auto-Rust utilizes Rust's powerful procedural macro system to inject code at compile time. When you use the `#[llm_tool]` macro:

1. **Parsing:** The macro parses the annotated function's signature, including its name, arguments, return type, and any doc comments.

2. **Context Extraction:** It extracts the code within your project, providing some context for the LLM to understand the surrounding code and project better.

3. **Prompt Engineering:** It constructs a prompt that includes the extracted information. This prompt is carefully designed to guide the LLM in generating a relevant and correct Rust function implementation.

4. **LLM Interaction:** It sends the generated prompt to an LLM API. 

5. **Code Insertion:** The LLM's response, which contains the generated Rust code, is inserted directly into your codebase, replacing the `todo!()` placeholder.

## Limitations

* **LLM Non-Determinism:** LLM output can vary. You might need to experiment and iterate to achieve the desired results.
* **Limited Context:** Currently, Auto-Rust's context is primarily based on your function signature and doc comments. It has limited awareness of the broader project structure.
* **Dependency Management:** Auto-Rust does not automatically add `use` statements for newly introduced dependencies in the generated code.

## Contributing

We welcome contributions from the Rust community! If you encounter issues, have suggestions, or want to contribute to the development of `auto-rust`, please feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT and Apache-2.0 licenses. 
