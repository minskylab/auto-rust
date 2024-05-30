use auto_rust::llm_tool;

#[llm_tool]
/// This function returns the power of two of a number
fn power_two(number: u64) -> u64 {
    todo!()
}

fn main() {
    let res = power_two(4);

    println!("{:?}", res)
}
