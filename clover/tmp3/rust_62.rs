
fn derivative(xs: Vec<i32>) -> Vec<i32> {
    xs.iter()
        .enumerate()
        .skip(1)
        .map(|(i, &coef)| coef * (i as i32))
        .collect()
}

fn main() {
    // Example of using the derivative function
    let polynomial = vec![2, 3, 4]; // Represents 2 + 3x + 4x^2
    let derivative_poly = derivative(polynomial);
    println!("{:?}", derivative_poly);
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
