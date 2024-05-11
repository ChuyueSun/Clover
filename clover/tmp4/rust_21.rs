
fn rescale_to_unit(mut numbers: Vec<f32>) -> Vec<f32> {
    if numbers.is_empty() || numbers.len() == 1 {
        return numbers;
    }

    let min_value = *numbers.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_value = *numbers.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();

    if min_value != max_value {
        let range = max_value - min_value;
        numbers.iter_mut().for_each(|num| *num = (*num - min_value) / range);
    } else {
        numbers.iter_mut().for_each(|num| *num = 0.0);
    }

    numbers
}

fn main() {
    let numbers = vec![2.0, 5.0, 1.0, 3.0];
    let scaled_numbers = rescale_to_unit(numbers);
    println!("{:?}", scaled_numbers);
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
