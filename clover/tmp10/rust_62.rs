
fn derivative(xs: Vec<i32>) -> Vec<i32> {
    xs.iter()
        .enumerate()
        .skip(1) // Skip the first term as its derivative is 0.
        .map(|(i, &coef)| coef * i as i32) // Calculate the derivative.
        .collect() // Collect the terms into a vector.
}

fn main() {
    // Example usage:
    let coefficients = vec![5, 3, 2]; // Represents the polynomial 5 + 3x + 2x^2
    let derivative_coeffs = derivative(coefficients);
    // The output will be [3, 4] representing the polynomial 3 + 4x
    println!("{:?}", derivative_coeffs);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derivative() {
        assert!(derivative(vec![3, 1, 2, 4, 5]) == vec![1, 4, 12, 20]);
        assert!(derivative(vec![1, 2, 3]) == vec![2, 6]);
        assert!(derivative(vec![3, 2, 1]) == vec![2, 2]);
        assert!(derivative(vec![3, 2, 1, 0, 4]) == vec![2, 2, 0, 16]);
        assert!(derivative(vec![1]) == vec![]);
    }

}
