
fn has_close_elements(numbers: Vec<f32>, threshold: f32) -> bool {
    for (i, &num1) in numbers.iter().enumerate() {
        for &num2 in numbers.iter().skip(i + 1) {
            if (num1 - num2).abs() < threshold {
                return true;
            }
        }
    }
    false
}

fn main() {
    // Example usage:
    let numbers = vec![1.5, 3.2, 5.6, 10.0];
    let threshold = 1.7;
    let result = has_close_elements(numbers, threshold);
    println!("Has close elements: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_close_elements() {
        assert_eq!(has_close_elements(vec![11.0, 2.0, 3.9, 4.0, 5.0, 2.2], 0.3), true);
        assert_eq!(has_close_elements(vec![1.0, 2.0, 3.9, 4.0, 5.0, 2.2], 0.05), false);
        assert_eq!(has_close_elements(vec![1.0, 2.0, 5.9, 4.0, 5.0], 0.95), true);
        assert_eq!(has_close_elements(vec![1.0, 2.0, 5.9, 4.0, 5.0], 0.8), false);
        assert_eq!(has_close_elements(vec![1.0, 2.0, 3.0, 4.0, 5.0, 2.0], 0.1), true);
        assert_eq!(has_close_elements(vec![1.1, 2.2, 3.1, 4.1, 5.1], 1.0), true);
        assert_eq!(has_close_elements(vec![1.1, 2.2, 3.1, 4.1, 5.1], 0.5), false);
    }

}
