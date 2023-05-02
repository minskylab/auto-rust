use auto_rust::implement;

implement!(fn my_ip() -> String);

fn main() {
    let result = my_ip();
    println!("result: {}", result);
}
