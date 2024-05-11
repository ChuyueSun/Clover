
fn find_closest_elements(mut numbers: Vec<f32>) -> (f32, f32) {
    if numbers.len() < 2 {
        panic!("The list should contain at least two numbers.");
    }

    numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut closest_pair = (numbers[0], numbers[1]);
    let mut smallest_difference = (numbers[1] - numbers[0]).abs();

    for window in numbers.windows(2) {
        let difference = (window[1] - window[0]).abs();
        if difference < smallest_difference {
            smallest_difference = difference;
            closest_pair = (window[0], window[1]);
        }
    }

    closest_pair
}

fn main() {
    let numbers = vec![3.5, 2.8, 4.2, 5.1];
    let (num1, num2) = find_closest_elements(numbers);
    println!("The closest pair is: ({}, {})", num1, num2);
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
