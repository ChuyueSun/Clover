
fn find_closest_elements(mut numbers: Vec<f32>) -> (f32, f32) {
    if numbers.len() < 2 {
        panic!("The list must contain at least two elements.");
    }

    numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut smallest_difference = f32::MAX;
    let mut closest_pair = (0.0, 0.0);

    for pair in numbers.windows(2) {
        let diff = pair[1] - pair[0];
        if diff < smallest_difference {
            smallest_difference = diff;
            closest_pair = (pair[0], pair[1]);
        }
    }

    closest_pair
}

fn main() {
    let numbers = vec![3.4, 7.6, 12.9, 15.2, 14.7, 5.3, 8.1];
    let (num1, num2) = find_closest_elements(numbers);
    println!("The closest elements are {} and {}", num1, num2);
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
