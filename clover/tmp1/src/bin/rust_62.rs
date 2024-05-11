
fn derivative(xs: Vec<i32>) -> Vec<i32> {
    xs.into_iter()
        .enumerate()
        .skip(1) // Skip the constant term
        .map(|(i, coef)| coef * i as i32)
        .collect()
}

fn main() {
    let poly_coeffs = vec![5, 3, 2]; // Represents the polynomial 5 + 3x + 2x^2
    let derivative_coeffs = derivative(poly_coeffs);
    println!("{:?}", derivative_coeffs); // Should print [3, 4]
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
