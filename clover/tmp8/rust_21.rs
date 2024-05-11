
fn rescale_to_unit(numbers: Vec<f32>) -> Vec<f32> {
    if numbers.is_empty() {
        return Vec::new();
    }

    let min_number = numbers.iter().cloned().fold(f32::INFINITY, f32::min);
    let max_number = numbers.iter().cloned().fold(f32::NEG_INFINITY, f32::max);

    if (max_number - min_number).abs() < f32::EPSILON {
        return vec![0.0; numbers.len()];
    }

    numbers
        .iter()
        .map(|&num| (num - min_number) / (max_number - min_number))
        .collect()
}

fn main() {
    // Example usage:
    let numbers = vec![3.0, 6.0, 2.0, 9.0];
    let rescaled_numbers = rescale_to_unit(numbers);
    println!("{:?}", rescaled_numbers);
}
`
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
