
fn sum_product(numbers: Vec<i32>) -> (i32, i32) {
    let sum = numbers.iter().sum();
    let product = numbers.iter().fold(1, |acc, &x| acc * x);
    (sum, product)
}

fn main() {
    // Example usage
    let numbers = vec![1, 2, 3, 4];
    let (sum, product) = sum_product(numbers);
    println!("Sum: {}, Product: {}", sum, product);
}

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_sum_product() {
        assert!(sum_product(vec![]) == (0, 1));
        assert!(sum_product(vec![1, 1, 1]) == (3, 1));
        assert!(sum_product(vec![100, 0]) == (100, 0));
        assert!(sum_product(vec![3, 5, 7]) == (3 + 5 + 7, 3 * 5 * 7));
        assert!(sum_product(vec![10]) == (10, 10));
    }

}
