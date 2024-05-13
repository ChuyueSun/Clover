fn count_nums(n: Vec<i32>) -> i32 {
    n.iter()
     .filter(|&&x| sum_of_digits(x) > 0)
     .count() as i32
}

fn sum_of_digits(mut x: i32) -> i32 {
    let mut sum = 0;
    let negative = x < 0;
    if negative {
        x = x.abs();
    }

    while x > 0 {
        let digit = x % 10;
        if negative && sum == 0 {
            // Make the first digit negative
            sum += -digit;
        } else {
            sum += digit;
        }
        x /= 10;
    }
    
    sum
}
fn main() {
    let numbers = vec![15, -23, 0, 5, -9];
    println!("The count is: {}", count_nums(numbers));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_nums() {
        assert!(count_nums(vec![]) == 0);
        assert!(count_nums(vec![-1, -2, 0]) == 0);
        assert!(count_nums(vec![1, 1, 2, -2, 3, 4, 5]) == 6);
        println!("The count is: {}", count_nums(vec![12, 23, 34, -45, -56, 0]));

        assert!(count_nums(vec![1, 6, 9, -6, 0, 1, 5]) == 5);
        assert!(count_nums(vec![1, 100, 98, -7, 1, -1]) == 4);
        assert!(count_nums(vec![12, 23, 34, -45, -56, 0]) == 5);
        assert!(count_nums(vec![-0, 1]) == 1);
        assert!(count_nums(vec![1]) == 1);
    }

}
