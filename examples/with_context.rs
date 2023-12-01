use auto_rust::auto_implement;

#[auto_implement(
    context = "This algorithm works by calculating the area of a circle using the trigonometric formula for the area of a triangle. Don't
    use powi, use powf instead. Don't use the sqrt function, use the hypot function instead. Don't use the sin function, use the sin_cos function instead. Don't use the cos function, use the sin_cos function instead"
)]
fn calculate_pi_with_n_iterations(n: u64) -> f64 {
    todo!()
}

fn main() {
    let result = calculate_pi_with_n_iterations(100_000);
    println!("pi: {}", result);
    // assert_eq!(result, 3.0418396189294032);
}
