
/// Calculates the largest prime factor of a given non-prime number n.
///
/// # Arguments
///
/// * `n` - A non-prime integer greater than 1.
///
/// # Returns
///
/// * `i32` - The largest prime factor of `n`.
fn largest_prime_factor(mut n: i32) -> i32 {
    // Handle even numbers first
    while n % 2 == 0 {
        n /= 2;
    }
    if n == 1 {
        return 2;
    }

    // n must be odd at this point, so a skip of 2 (i += 2) can be used
    let mut largest_prime = 3;
    let mut i = 3;
    while i * i <= n {
        // While i divides n, save i and divide n
        while n % i == 0 {
            largest_prime = i;
            n /= i;
        }
        i += 2;
    }

    // This condition is to handle the case when n is a prime number greater than 2
    if n > 2 {
        largest_prime = n;
    }

    largest_prime
}

fn main() {
    // Test the function
    let number = 13195; // For demonstration purpose, we know 13195 is not prime.
    println!("The largest prime factor of {} is {}", number, largest_prime_factor(number));
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
