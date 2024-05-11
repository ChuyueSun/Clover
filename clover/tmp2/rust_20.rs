
fn find_closest_elements(mut numbers: Vec<f32>) -> (f32, f32) {
    if numbers.len() < 2 {
        panic!("The list must contain at least two numbers");
    }

    numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut closest_pair = (numbers[0], numbers[1]);
    let mut min_diff = (numbers[1] - numbers[0]).abs();

    for window in numbers.windows(2) {
        let diff = (window[1] - window[0]).abs();
        if diff < min_diff {
            closest_pair = (window[0], window[1]);
            min_diff = diff;
        }
    }

    closest_pair
}

fn main() {
    let numbers = vec![2.5, 4.1, 3.3, 5.8];
    let closest_pair = find_closest_elements(numbers);
    println!("The closest pair is {:?}", closest_pair);
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
