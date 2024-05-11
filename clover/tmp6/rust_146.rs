
fn special_filter(nums: Vec<i32>) -> usize {
    nums.iter()
        .filter(|&&num| num > 10 && is_first_last_digit_odd(num))
        .count()
}

fn is_first_last_digit_odd(num: i32) -> bool {
    if num <= 0 {
        return false;
    }
    let last_digit = num % 10;
    if last_digit % 2 == 0 {
        return false;
    }
    let mut first_digit = num;
    while first_digit >= 10 {
        first_digit /= 10;
    }
    first_digit % 2 != 0
}

fn main() {
    // Example usage:
    let nums = vec![11, 23, 34, 55, 71];
    println!("Count: {}", special_filter(nums));
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
