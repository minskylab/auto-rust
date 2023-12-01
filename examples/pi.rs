use auto_rust::auto_implement;

#[auto_implement]
fn calculate_pi_with_n_iterations(n: u64) -> f64 {
    todo!()
}

fn main() {
    let result = calculate_pi_with_n_iterations(100_000);
    println!("pi: {}", result);
    // assert_eq!(result, 3.0418396189294032);
}
