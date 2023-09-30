use auto_rust::auto_implement;

#[auto_implement]
#[doc = "This function checks if a string is an email without regex."]
fn is_email(input: String) -> bool {
    todo!()
}

fn main() {
    let result = is_email("bregy@minsky.cc".to_string());
    println!("result: {}", result);
}
