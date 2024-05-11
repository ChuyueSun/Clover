
fn special_filter(nums: Vec<i32>) -> usize {
    nums.iter()
        .filter(|&&num| num > 10 && is_first_and_last_digit_odd(num))
        .count()
}

fn is_first_and_last_digit_odd(num: i32) -> bool {
    if num < 0 {
        return false;
    }
    let num_str = num.to_string();
    let bytes = num_str.as_bytes();
    let first_digit = bytes[0] as char;
    let last_digit = bytes[bytes.len() - 1] as char;
    
    matches!(first_digit, '1' | '3' | '5' | '7' | '9') && matches!(last_digit, '1' | '3' | '5' | '7' | '9')
}

fn main() {
    let nums = vec![11, 29, 22, 47, 19, 88, 123, 37];
    println!("Number of special numbers: {}", special_filter(nums));
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
