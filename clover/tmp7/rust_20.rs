
fn find_closest_elements(mut numbers: Vec<f32>) -> (f32, f32) {
    numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
    let mut closest_pair = (numbers[0], numbers[1]);
    let mut min_distance = (numbers[1] - numbers[0]).abs();

    for i in 1..numbers.len() - 1 {
        let distance = (numbers[i + 1] - numbers[i]).abs();
        if distance < min_distance {
            min_distance = distance;
            closest_pair = (numbers[i], numbers[i + 1]);
        }
    }

    closest_pair
}

fn main() {
    // Example usage:
    let numbers = vec![3.5, 1.7, 4.1, 2.0, 1.9];
    let (num1, num2) = find_closest_elements(numbers);
    println!("The closest pair of numbers are {} and {}", num1, num2);
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
