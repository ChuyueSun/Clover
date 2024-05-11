
/// Returns the largest prime factor of a given number `n`.
/// Assumes `n` is an integer greater than 1 and is not a prime.
///
/// # Arguments
///
/// * `n` - An integer value for which to find the largest prime factor.
///
/// # Example
///
/// ```
/// let largest_prime = largest_prime_factor(28);
/// assert_eq!(largest_prime, 7);
/// ```
fn largest_prime_factor(mut n: i32) -> i32 {
    let mut largest_factor = 1;
    let mut factor = 2;

    while factor * factor <= n {
        while n % factor == 0 {
            n /= factor;
            largest_factor = factor;
        }
        factor += if factor == 2 { 1 } else { 2 }; // Increment factor, taking into account that 2 is the only even prime
    }

    if n > 1 {
        n // If there is any remainder this is prime
    } else {
        largest_factor
    }
}

fn main() {
    // Example usage:
    let n = 28;
    println!("The largest prime factor of {} is {}", n, largest_prime_factor(n));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_prime_factor() {
        assert!(largest_prime_factor(15) == 5);
        assert!(largest_prime_factor(27) == 3);
        assert!(largest_prime_factor(63) == 7);
        assert!(largest_prime_factor(330) == 11);
        assert!(largest_prime_factor(13195) == 29);
    }

}
