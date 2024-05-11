
use primal::Primes;

/// Finds the largest prime in a list and returns the sum of its digits.
///
/// # Arguments
///
/// * `lst` - A vector of integers to search through.
///
/// # Returns
///
/// * The sum of the digits of the largest prime number found in the list.
/// If no prime number is found, returns 0.
///
/// # Examples
///
/// ```
/// let primes = vec![4, 6, 8, 11, 13];
/// let sum_of_digits = skjkasdkd(primes);
/// assert_eq!(sum_of_digits, 4); // 13 is the largest prime, its digits sum up to 4
/// ```
fn skjkasdkd(lst: Vec<i32>) -> i32 {
    fn is_prime(n: i32) -> bool {
        if n <= 1 {
            return false;
        }
        if n <= 3 {
            return true;
        }
        if n % 2 == 0 || n % 3 == 0 {
            return false;
        }
        let mut i = 5;
        while i * i <= n {
            if n % i == 0 || n % (i + 2) == 0 {
                return false;
            }
            i += 6;
        }
        true
    }

    fn sum_of_digits(mut n: i32) -> i32 {
        let mut sum = 0;
        while n > 0 {
            sum += n % 10;
            n /= 10;
        }
        sum
    }

    lst.iter()
        .filter(|&&x| is_prime(x))
        .max()
        .map_or(0, |&max_prime| sum_of_digits(max_prime))
}

fn main() {
    let primes = vec![4, 6, 8, 11, 13];
    let sum_of_digits = skjkasdkd(primes);
    println!("Sum of digits of the largest prime: {}", sum_of_digits);
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
