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
