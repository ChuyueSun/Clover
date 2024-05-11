
fn count_nums(n: Vec<i32>) -> i32 {
    n.iter()
     .filter(|&&x| {
         let sum_of_digits = if x < 0 {
             x.to_string().chars().skip(1).fold(0, |acc, digit| acc + digit.to_digit(10).unwrap())
             - x.to_string().chars().nth(1).unwrap().to_digit(10).unwrap() * 2
         } else {
             x.to_string().chars().fold(0, |acc, digit| acc + digit.to_digit(10).unwrap())
         };
         sum_of_digits > 0
     })
     .count() as i32
}

fn main() {
    let nums = vec![10, -21, 54, -123, 0];
    println!("Count: {}", count_nums(nums));
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
