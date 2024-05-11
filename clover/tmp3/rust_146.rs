
fn special_filter(nums: Vec<i32>) -> usize {
    nums.iter().filter(|&&num| {
        // Check that the number is greater than 10
        if num > 10 {
            let digits = num.to_string();
            let first_digit = digits.chars().nth(0).unwrap().to_digit(10).unwrap();
            let last_digit = digits.chars().last().unwrap().to_digit(10).unwrap();
            
            // Check if the first and last digits are both odd
            first_digit % 2 != 0 && last_digit % 2 != 0
        } else {
            false
        }
    }).count()
}

fn main() {
    // Example usage:
    let numbers = vec![14, 23, 45, 92, 101, 35, 13];
    let result = special_filter(numbers);
    println!("The number of special filtered elements is: {}", result);
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
