
fn rescale_to_unit(numbers: Vec<f32>) -> Vec<f32> {
    let min = numbers.iter().cloned().fold(f32::INFINITY, f32::min);
    let max = numbers.iter().cloned().fold(f32::NEG_INFINITY, f32::max);

    if (max - min).abs() < f32::EPSILON {
        // If all numbers are the same, we can't perform the scaling as described.
        // It is undefined what to return or if returning an error is more appropriate.
        // Here, we choose to return the input as the output.
        return numbers;
    }

    numbers.into_iter()
        .map(|x| (x - min) / (max - min))
        .collect()
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
