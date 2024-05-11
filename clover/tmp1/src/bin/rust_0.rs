
fn has_close_elements(numbers: Vec<f32>, threshold: f32) -> bool {
    for i in 0..numbers.len() {
        for j in (i + 1)..numbers.len() {
            if (numbers[i] - numbers[j]).abs() < threshold {
                return true;
            }
        }
    }
    false
}

fn main() {
    // Example usage:
    let numbers = vec![1.5, 3.2, 5.7, 2.1];
    let threshold = 0.8;
    println!("Has close elements: {}", has_close_elements(numbers, threshold));
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
