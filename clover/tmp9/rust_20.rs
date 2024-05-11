
fn find_closest_elements(mut numbers: Vec<f32>) -> (f32, f32) {
    if numbers.len() < 2 {
        panic!("The vector must have at least two elements.");
    }

    numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut closest_pair = (numbers[0], numbers[1]);
    let mut smallest_difference = (numbers[1] - numbers[0]).abs();

    for i in 0..numbers.len() - 1 {
        let diff = (numbers[i + 1] - numbers[i]).abs();
        if diff < smallest_difference {
            smallest_difference = diff;
            closest_pair = (numbers[i], numbers[i + 1]);
        }
    }

    closest_pair
}

fn main() {
    // Example usage:
    let numbers = vec![5.5, 3.1, 2.2, 8.9];
    let (a, b) = find_closest_elements(numbers);
    println!("The closest pair is: ({}, {})", a, b);
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
