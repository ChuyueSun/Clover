
fn rolling_max(numbers: Vec<i32>) -> Vec<i32> {
    let mut max_rolling_numbers = Vec::with_capacity(numbers.len());
    let mut current_max = i32::MIN;
    for &number in numbers.iter() {
        if number > current_max {
            current_max = number;
        }
        max_rolling_numbers.push(current_max);
    }
    max_rolling_numbers
}

fn main() {
    // Example usage:
    let numbers = vec![1, 2, 5, 3, 4];
    let roll_max = rolling_max(numbers);
    println!("{:?}", roll_max); // Should print: [1, 2, 5, 5, 5]
}

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_rolling_max() {
        assert!(rolling_max(vec![]) == vec![]);
        assert!(rolling_max(vec![1, 2, 3, 4]) == vec![1, 2, 3, 4]);
        assert!(rolling_max(vec![4, 3, 2, 1]) == vec![4, 4, 4, 4]);
        assert!(rolling_max(vec![3, 2, 3, 100, 3]) == vec![3, 3, 3, 100, 100]);
    }

}
