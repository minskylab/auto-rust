use auto_rust::auto_implement;

#[auto_implement]
/// This function calculates pi with a reliable algorithm.
fn calculate_pi_with_n_iterations(n: u32) -> f64 {
    todo!()
}

fn main() {
    let result = calculate_pi_with_n_iterations(100);
    println!("pi: {}", result);
    // assert_eq!(result, 3.0418396189294032);
}
