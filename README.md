# Auto Rust

`auto-rust` is an experimental project that aims to automatically generate Rust code with LLM (Large Language Models) during compilation, utilizing procedural macros.

## ⚠️ Warning

Please note that Auto-Rust is currently under development and is not yet suitable for production use. While you are welcome to try it out and provide feedback, we caution that it may have an incomplete implementation and may not function as intended.

## Installation

```toml
[dependencies]
auto-rust = "0.1.0"
```

## Example

```rust
use auto_rust::auto_implement;

#[auto_implement]
#[doc = "This function calculates if the input is a valid email address without use regex."]
fn is_email(input: String) -> bool {
    todo!()
}

fn main() {
    let result = is_email("bregyminsky.cc".to_string());
    println!("result: {}", result);
}
```

## Contributing

Contributions are welcome. Feel free to open an issue if you have any questions or want to suggest an improvement.
