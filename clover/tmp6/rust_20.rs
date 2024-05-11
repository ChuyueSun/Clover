
fn find_closest_elements(numbers: Vec<f32>) -> (f32, f32) {
    assert!(numbers.len() >= 2, "The length of the numbers list must be at least two.");
    
    let mut sorted_numbers = numbers.clone();
    sorted_numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut min_difference = f32::MAX;
    let mut result = (0.0, 0.0);

    for window in sorted_numbers.windows(2) {
        if let [first, second] = window {
            let difference = second - first;
            if difference < min_difference {
                min_difference = difference;
                result = (*first, *second);
            }
        }
    }

    result
}

fn main() {
    let numbers = vec![5.2, 3.8, 6.4, 2.1, 5.0];
    let (num1, num2) = find_closest_elements(numbers);
    println!("The closest elements are: {} and {}", num1, num2);
}

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_find_closest_elements() {
        assert!(find_closest_elements(vec![1.0, 2.0, 3.9, 4.0, 5.0, 2.2]) == (3.9, 4.0));
        assert!(find_closest_elements(vec![1.0, 2.0, 5.9, 4.0, 5.0]) == (5.0, 5.9));
        assert!(find_closest_elements(vec![1.0, 2.0, 3.0, 4.0, 5.0, 2.2]) == (2.0, 2.2));
        assert!(find_closest_elements(vec![1.0, 2.0, 3.0, 4.0, 5.0, 2.0]) == (2.0, 2.0));
        assert!(find_closest_elements(vec![1.1, 2.2, 3.1, 4.1, 5.1]) == (2.2, 3.1));
    }

}
