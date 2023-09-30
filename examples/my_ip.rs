use auto_rust::auto_implement;

#[auto_implement]
fn my_ip() -> String {
    todo!()
}

fn main() {
    let result = my_ip();
    println!("result: {}", result);
}
