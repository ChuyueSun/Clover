
fn find_closest_elements(numbers: Vec<f32>) -> (f32, f32) {
    if numbers.len() < 2 {
        panic!("The list of numbers must contain at least two elements.");
    }

    let mut numbers_sorted = numbers.clone();
    numbers_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut closest_pair = (numbers_sorted[0], numbers_sorted[1]);
    let mut min_difference = (numbers_sorted[1] - numbers_sorted[0]).abs();

    for window in numbers_sorted.windows(2) {
        let difference = (window[1] - window[0]).abs();
        if difference < min_difference {
            min_difference = difference;
            closest_pair = (window[0], window[1]);
        }
    }

    closest_pair
}

fn main() {
    // Example usage:
    let numbers = vec![3.4, 5.1, 4.8, 3.9];
    let (a, b) = find_closest_elements(numbers);
    println!("The closest pair is ({}, {})", a, b);
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
