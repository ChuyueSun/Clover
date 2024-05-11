
fn derivative(xs: Vec<i32>) -> Vec<i32> {
    xs.iter()
        .enumerate()
        .skip(1)
        .map(|(i, &coef)| coef * i as i32)
        .collect()
}

fn main() {
    // Example usage:
    let coeffs = vec![5, 3, 1];  // represents the polynomial 5 + 3*x + x^2
    let deriv_coeffs = derivative(coeffs);
    println!("{:?}", deriv_coeffs);  // should print [3, 2]
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
