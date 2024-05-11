
fn find_closest_elements(mut numbers: Vec<f32>) -> (f32, f32) {
    if numbers.len() < 2 {
        panic!("The list of numbers must contain at least two elements.");
    }

    numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut closest_pair = (numbers[0], numbers[1]);
    let mut smallest_difference = (numbers[1] - numbers[0]).abs();

    for i in 1..numbers.len() - 1 {
        let diff = (numbers[i + 1] - numbers[i]).abs();
        if diff < smallest_difference {
            smallest_difference = diff;
            closest_pair = (numbers[i], numbers[i + 1]);
        }
    }

    closest_pair
}

fn main() {
    let numbers = vec![4.5, 11.0, 8.5, 3.0, 10.5];
    let (a, b) = find_closest_elements(numbers);
    println!("Closest pair is: ({}, {})", a, b);
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
