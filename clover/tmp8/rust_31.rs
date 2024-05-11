
/// Check if the given number is prime.
///
/// # Arguments
/// * `n`: The number to check.
///
/// # Returns
/// * `true` if `n` is a prime number.
/// * `false` otherwise.
///
/// # Examples
/// ```
/// assert_eq!(is_prime(2), true);
/// assert_eq!(is_prime(11), true);
/// assert_eq!(is_prime(4), false);
/// ```
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

// To use the function, you may write your tests or checks here, for example:
fn main() {
    println!("Is 2 prime? {}", is_prime(2));
    println!("Is 11 prime? {}", is_prime(11));
    println!("Is 4 prime? {}", is_prime(4));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        assert!(is_prime(6) == false);
        assert!(is_prime(101) == true);
        assert!(is_prime(11) == true);
        assert!(is_prime(13441) == true);
        assert!(is_prime(61) == true);
        assert!(is_prime(4) == false);
        assert!(is_prime(1) == false);
        assert!(is_prime(5) == true);
        assert!(is_prime(11) == true);
        assert!(is_prime(17) == true);
        assert!(is_prime(5 * 17) == false);
        assert!(is_prime(11 * 7) == false);
        assert!(is_prime(13441 * 19) == false);
    }

}
