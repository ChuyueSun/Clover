
fn special_filter(nums: Vec<i32>) -> usize {
    nums.into_iter()
        .filter(|&n| {
            n > 10 && {
                let digits = n.to_string().into_bytes();
                let first_digit = digits.first().unwrap() - b'0';
                let last_digit = digits.last().unwrap() - b'0';
                first_digit % 2 == 1 && last_digit % 2 == 1
            }
        })
        .count()
}

fn main() {
    // Example usage
    let numbers = vec![13, 22, 83, 46, 57, 29];
    let count = special_filter(numbers);
    println!("The count of special numbers is: {}", count);
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
