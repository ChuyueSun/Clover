
fn special_filter(nums: Vec<i32>) -> usize {
    nums.iter()
        .filter(|&&num| {
            // Check if the number is greater than 10
            if num <= 10 {
                return false;
            }

            // Cast number to absolute and to string to check the digits
            let num_str = num.abs().to_string();
            // Check if both first and last characters are in the set of odd digits
            if let (Some(first), Some(last)) = (num_str.chars().next(), num_str.chars().last()) {
                let odd_digits = ['1', '3', '5', '7', '9'];
                odd_digits.contains(&first) && odd_digits.contains(&last)
            } else {
                false // If the number is a single digit, this will occur, and the number is not > 10
            }
        })
        .count()
}

fn main() {
    // Example usage:
    let numbers = vec![13, 26, 15, 99, 88, 123, 135];
    let count = special_filter(numbers);
    println!("Count of special numbers: {}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_special_filter() {
        assert_eq!(special_filter(vec![5, -2, 1, -5]), 0);
        assert_eq!(special_filter(vec![15, -73, 14, -15]), 1);
        assert_eq!(special_filter(vec![33, -2, -3, 45, 21, 109]), 2);
        assert_eq!(special_filter(vec![43, -12, 93, 125, 121, 109]), 4);
        assert_eq!(special_filter(vec![71, -2, -33, 75, 21, 19]), 3);
        assert_eq!(special_filter(vec![1]), 0);
        assert_eq!(special_filter(vec![]), 0);
    }

}
