
fn count_nums(n: Vec<i32>) -> usize {
    n.iter()
        .filter(|&&x| sum_of_digits(x) > 0)
        .count()
}

fn sum_of_digits(mut num: i32) -> i32 {
    let mut sum = 0;
    let negative = num < 0;
    while num != 0 {
        let digit = num % 10;
        sum += if negative { -digit.abs() } else { digit };
        num /= 10;
    }
    sum
}

fn main() {
    // Example Test
    let numbers = vec![123, -456, 0, 9999];
    println!("The count is {}", count_nums(numbers));
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
