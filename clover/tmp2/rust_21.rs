
fn rescale_to_unit(mut numbers: Vec<f32>) -> Vec<f32> {
    if let (Some(min_val), Some(max_val)) = (numbers.iter().copied().min_by(|a, b| a.partial_cmp(b).unwrap()), numbers.iter().copied().max_by(|a, b| a.partial_cmp(b).unwrap())) {
        let range = max_val - min_val;
        if range != 0.0 {
            numbers.iter_mut().for_each(|n| *n = (*n - min_val) / range);
        }
    }
    numbers
}

fn main() {
    // Example usage:
    let numbers = vec![2.0, 5.0, 1.0, 8.0, 3.0];
    println!("{:?}", rescale_to_unit(numbers)); // Should print something like: [0.14285715, 0.5714286, 0.0, 1.0, 0.2857143]
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn test_rescale_to_unit() {
        assert!(rescale_to_unit(vec![2.0, 49.9]) == [0.0, 1.0]);
        assert!(rescale_to_unit(vec![100.0, 49.9]) == [1.0, 0.0]);
        assert!(rescale_to_unit(vec![1.0, 2.0, 3.0, 4.0, 5.0]) == [0.0, 0.25, 0.5, 0.75, 1.0]);
        assert!(rescale_to_unit(vec![2.0, 1.0, 5.0, 3.0, 4.0]) == [0.25, 0.0, 1.0, 0.5, 0.75]);
        assert!(rescale_to_unit(vec![12.0, 11.0, 15.0, 13.0, 14.0]) == [0.25, 0.0, 1.0, 0.5, 0.75]);
    }

}