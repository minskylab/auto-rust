# Auto Rust

`auto-rust` is an experimental project that aims to automatically generate Rust code with LLM (Large Language Models) during compilation, utilizing procedural macros.

## Example

```rust
use auto_rust::implement;

implement!(fn is_email(input: Into<String>) -> bool);

fn main() {
    let result = is_email("hello@minsky.cc");
    assert_eq!(result, true);

    let result = is_email("hello@minsky");
    assert_eq!(result, false);
}
```
