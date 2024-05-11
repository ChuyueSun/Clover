
fn derivative(xs: Vec<i32>) -> Vec<i32> {
    xs.into_iter()
        .enumerate()
        .skip(1) // Skip the constant term, its derivative is 0.
        .map(|(i, coefficient)| coefficient * i as i32) // i is the power of x
        .collect()
}

fn main() {
    // Example usage:
    let polynomial = vec![1, 2, 3]; // Represents 1 + 2x + 3x^2
    let derivative_poly = derivative(polynomial);
    println!("{:?}", derivative_poly); // Should print [2, 6]
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
