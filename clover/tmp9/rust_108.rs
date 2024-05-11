
fn count_nums(n: Vec<i32>) -> usize {
    n.into_iter()
        .filter(|&num| {
            num.to_string()
                .chars()
                .filter_map(|d| match d.to_digit(10) {
                    Some(digit) => Some(if num.is_negative() && d == '-' { 0 } else { digit as i32 }),
                    None => None,
                })
                .sum::<i32>()
                > 0
        })
        .count()
}

fn main() {
    // Example usage:
    let nums = vec![123, -456, 0, 789, -12];
    println!("The count of numbers with sum of digits > 0 is: {}", count_nums(nums));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_nums() {
        assert!(count_nums(vec![]) == 0);
        assert!(count_nums(vec![-1, -2, 0]) == 0);
        assert!(count_nums(vec![1, 1, 2, -2, 3, 4, 5]) == 6);
        assert!(count_nums(vec![1, 6, 9, -6, 0, 1, 5]) == 5);
        assert!(count_nums(vec![1, 100, 98, -7, 1, -1]) == 4);
        assert!(count_nums(vec![12, 23, 34, -45, -56, 0]) == 5);
        assert!(count_nums(vec![-0, 1]) == 1);
        assert!(count_nums(vec![1]) == 1);
    }

}
