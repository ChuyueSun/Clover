
fn special_filter(nums: Vec<i32>) -> usize {
    nums.iter()
        .filter(|&&num| {
            if num <= 10 {
                return false;
            }
            let num_str = num.abs().to_string();
            let digits: Vec<char> = num_str.chars().collect();
            let first_digit = digits.first().unwrap();
            let last_digit = digits.last().unwrap();
            "13579".contains(*first_digit) && "13579".contains(*last_digit)
        })
        .count()
}

fn main() {
    let nums = vec![11, 29, 72, 135, 33, 2, 9, 111];
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
