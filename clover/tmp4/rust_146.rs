
fn special_filter(nums: Vec<i32>) -> usize {
    nums.iter()
        .filter(|&&num| num > 10 && is_first_and_last_digit_odd(num))
        .count()
}

fn is_first_and_last_digit_odd(num: i32) -> bool {
    let digits: Vec<_> = num.to_string().chars().collect();
    if let (Some(first), Some(last)) = (digits.first(), digits.last()) {
        return is_digit_odd(*first) && is_digit_odd(*last);
    }
    false
}

fn is_digit_odd(digit: char) -> bool {
    matches!(digit, '1' | '3' | '5' | '7' | '9')
}

fn main() {
    let nums = vec![11, 22, 23, 47, 35, 68, 93, 515];
    let count = special_filter(nums);
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
