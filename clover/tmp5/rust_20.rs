
fn find_closest_elements(mut numbers: Vec<f32>) -> (f32, f32) {
    // Ensure there are at least two numbers in the list
    if numbers.len() < 2 {
        panic!("List must contain at least two numbers");
    }

    // Sort the numbers to make it easier to find the closest pair
    numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut closest_pair = (numbers[0], numbers[1]);
    let mut smallest_difference = (numbers[1] - numbers[0]).abs();

    // Iterate over the sorted list to find the closest pair
    for i in 1..numbers.len() - 1 {
        let difference = (numbers[i + 1] - numbers[i]).abs();
        if difference < smallest_difference {
            smallest_difference = difference;
            closest_pair = (numbers[i], numbers[i + 1]);
        }
    }

    closest_pair
}

fn main() {
    // Example usage:
    let elements = vec![5.3, 2.7, 3.9, 4.1, 7.2];
    let closest = find_closest_elements(elements);
    println!("The closest pair is: {:?}", closest);
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
