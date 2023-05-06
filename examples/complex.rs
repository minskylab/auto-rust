use auto_rust::implement;

implement!(fn is_email(input: String) -> bool);

fn main() {
    let result = is_email("bregy@minsky.cc".to_string());
    println!("result: {}", result);
}
