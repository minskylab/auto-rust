use auto_rust::auto_implement;

#[auto_implement]
/// Only return the string literal of the quine
/// Remember to use `to_string()` to convert it to a string
fn calculate_rust_quine() -> String {
    todo!()
}

fn main() {
    let quine = calculate_rust_quine();

    print!("{}", quine)
}
