
fn has_close_elements(numbers: Vec<f32>, threshold: f32) -> bool {
    for i in 0..numbers.len() {
        for j in (i+1)..numbers.len() {
            if (numbers[j] - numbers[i]).abs() < threshold {
                return true;
            }
        }
    }
    false
}

fn main() {
    let elements = vec![1.5, 3.2, 5.8];
    let thresh = 1.0;

    println!("Do any two numbers lie within the threshold? {}", has_close_elements(elements, thresh));
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
