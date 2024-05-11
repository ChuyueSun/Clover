
fn special_filter(nums: Vec<i32>) -> usize {
    nums.iter().filter(|&&num| {
        if num > 10 {
            let digits = num.to_string();
            let first_digit = digits.chars().nth(0).unwrap();
            let last_digit = digits.chars().last().unwrap();
            let odd_digits = ['1', '3', '5', '7', '9'];
            odd_digits.contains(&first_digit) && odd_digits.contains(&last_digit)
        } else {
            false
        }
    }).count()
}

fn main() {
    // Example usage:
    let nums = vec![11, 22, 23, 47, 58, 71, 85];
    let count = special_filter(nums);
    println!("There are {} special numbers.", count);
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
