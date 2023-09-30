use auto_rust::auto_implement;

#[auto_implement]
/// This function calculates pi with a reliable algorithm.
/// don't use 'powi' method
/// implement the best algorithm known by the humanity
/// use all of your knowledge about rust optimizations
fn calculate_pi_with_n_iterations(n: u64) -> f64 {
    todo!()
}

fn main() {
    let result = calculate_pi_with_n_iterations(100_000);
    println!("pi: {}", result);
    // assert_eq!(result, 3.0418396189294032);
}
