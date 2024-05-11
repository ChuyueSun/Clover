
fn rescale_to_unit(mut numbers: Vec<f32>) -> Vec<f32> {
    if numbers.len() < 2 {
        panic!("The list should have at least two elements.");
    }

    let min_value = *numbers.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_value = *numbers.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();

    let range = max_value - min_value;

    if range == 0.0 {
        panic!("The list should not have all elements the same.");
    }

    numbers.iter_mut().for_each(|x| {
        *x = (*x - min_value) / range;
    });

    numbers
}

fn main() {
    // Example usage:
    let numbers = vec![2.0, 5.0, 1.0, 4.0];
    let rescaled_numbers = rescale_to_unit(numbers);

    println!("{:?}", rescaled_numbers);
    // The output will be [0.25, 1.0, 0.0, 0.75]
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
