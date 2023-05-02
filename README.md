# Auto Rust

Auto rust is a experimental project to automatically generate rust code with LLM (Large Language Models) like GPT-4 at compile time using procedural macros.

## Example

```rust
use auto_rust::implement;

implement!(fn is_an_email(input: Into<String>) -> bool);

fn main() {
    let result = is_an_email("hello@minsky.cc");
    assert_eq!(result, true);

    let result = is_an_email("hello@minsky");
    assert_eq!(result, false);
}

```
