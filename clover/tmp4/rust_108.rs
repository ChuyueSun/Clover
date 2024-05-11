
fn count_nums(n: Vec<i32>) -> usize {
    n.iter().filter(|&&x| sum_of_digits(x) > 0).count()
}

fn sum_of_digits(mut num: i32) -> i32 {
    let mut sum = 0;
    let negative = num < 0;
    while num != 0 {
        sum += num % 10;
        num /= 10;
    }
    if negative { -sum.abs() } else { sum }
}

fn main() {
    // Example usage:
    let nums = vec![15, -47, 3, -5];
    let count = count_nums(nums);
    println!("The number of elements with a sum of digits > 0: {}", count);
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
