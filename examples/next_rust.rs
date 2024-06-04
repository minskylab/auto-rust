use auto_rust::llm_tool;

#[llm_tool]
/// This function returns n^2 + 1 of a number
fn n_squared_plus_one(n: u64) -> u64 {
    todo!()
}

fn main() {
    let res = n_squared_plus_one(4);

    println!("{:?}", res);
}
