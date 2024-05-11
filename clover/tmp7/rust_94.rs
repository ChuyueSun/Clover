
fn skjkasdkd(lst: Vec<i32>) -> i32 {
    fn is_prime(num: i32) -> bool {
        if num <= 1 {
            return false;
        }
        for i in 2..((num as f64).sqrt() as i32 + 1) {
            if num % i == 0 {
                return false;
            }
        }
        true
    }

    fn sum_of_digits(num: i32) -> i32 {
        num.to_string().chars().map(|c| c.to_digit(10).unwrap() as i32).sum()
    }

    lst.into_iter()
        .filter(|&x| is_prime(x))
        .max()
        .map_or(0, |max_prime| sum_of_digits(max_prime))
}

fn main() {
    // Sample usage
    let numbers = vec![10, 2, 3, 7, 11, 13];
    let result = skjkasdkd(numbers);
    println!("The sum of the digits of the largest prime is: {}", result);
    // Output should be 4 since 13 is the largest prime and its digit sum is 1 + 3 = 4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skjkasdkd() {
        assert!(
            skjkasdkd(vec![
                0, 3, 2, 1, 3, 5, 7, 4, 5, 5, 5, 2, 181, 32, 4, 32, 3, 2, 32, 324, 4, 3
            ]) == 10
        );
        assert!(
            skjkasdkd(vec![
                1, 0, 1, 8, 2, 4597, 2, 1, 3, 40, 1, 2, 1, 2, 4, 2, 5, 1
            ]) == 25
        );
        assert!(
            skjkasdkd(vec![
                1, 3, 1, 32, 5107, 34, 83278, 109, 163, 23, 2323, 32, 30, 1, 9, 3
            ]) == 13
        );
        assert!(skjkasdkd(vec![0, 724, 32, 71, 99, 32, 6, 0, 5, 91, 83, 0, 5, 6]) == 11);
        assert!(skjkasdkd(vec![0, 81, 12, 3, 1, 21]) == 3);
        assert!(skjkasdkd(vec![0, 8, 1, 2, 1, 7]) == 7);
        assert!(skjkasdkd(vec![8191]) == 19);
        assert!(skjkasdkd(vec![8191, 123456, 127, 7]) == 19);
        assert!(skjkasdkd(vec![127, 97, 8192]) == 10);
    }


}
