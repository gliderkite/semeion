/// Returns true only if the square root of the given number is an integer.
pub fn is_perfect_square(x: f64) -> bool {
    let square = x.sqrt();
    ((square * square) - x).abs() < std::f64::EPSILON
}
