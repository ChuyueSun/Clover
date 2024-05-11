
fn derivative(xs: Vec<i32>) -> Vec<i32> {
    xs.iter()
        .enumerate()
        .skip(1) // The first term has no x, its derivative is 0
        .map(|(index, &coeff)| coeff * (index as i32))
        .collect()
}

fn main() {
    let poly_coeffs = vec![1, -3, 2]; // Represents 1 - 3x + 2x^2
    let poly_derivative = derivative(poly_coeffs);
    println!("{:?}", poly_derivative); // Should print [-3, 4]
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
