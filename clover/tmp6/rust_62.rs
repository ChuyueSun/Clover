
fn derivative(xs: Vec<i32>) -> Vec<i32> {
    xs.iter()
        .enumerate()
        .skip(1)
        .map(|(i, &coeff)| coeff * (i as i32))
        .collect()
}

fn main() {
    // Example usage:
    let coefficients = vec![3, -2, 5, 7]; // Represents 3 - 2x + 5x^2 + 7x^3
    let derivative_coefficients = derivative(coefficients);
    println!("{:?}", derivative_coefficients); // Should print [-2, 10, 21]
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
