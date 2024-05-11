
fn largest_prime_factor(mut n: i32) -> i32 {
    // Start from the smallest prime number.
    let mut largest_factor = 2;

    // Divide n by 2 until it's odd, 2 is the only even prime.
    while n % 2 == 0 {
        n /= 2;
    }

    // Now that n is odd, we can skip even numbers. Start from 3 and iterate by 2 steps.
    let mut factor = 3;
    while factor * factor <= n {
        while n % factor == 0 {
            n /= factor;
            largest_factor = factor;
        }
        factor += 2;
    }

    // If n is now a prime number greater than 2, then it is the largest prime factor.
    if n > 2 {
        largest_factor = n;
    }

    largest_factor
}

fn main() {
    // Example usage:
    let number = 13195;
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
