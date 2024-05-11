
fn count_nums(n: Vec<i32>) -> usize {
    n.into_iter()
        .filter(|&num| {
            let sum_digits = if num >= 0 {
                num.to_string().chars().map(|d| d.to_digit(10).unwrap() as i32).sum()
            } else {
                num.to_string().chars().skip(1).fold(num.to_string().chars().next().unwrap().to_digit(10).unwrap() as i32 * -1, |acc, d| acc + d.to_digit(10).unwrap() as i32)
            };
            sum_digits > 0
        })
        .count()
}

fn main() {
    // Example usage:
    let numbers = vec![123, -246, 0, 19, -12];
    let result = count_nums(numbers);
    println!("The number of elements with a sum of digits > 0: {}", result);
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
