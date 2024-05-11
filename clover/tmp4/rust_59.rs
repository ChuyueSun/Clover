
fn largest_prime_factor(mut n: i32) -> i32 {
    // Initialize the max prime factor to the smallest prime number
    let mut max_prime = 2;

    // Divide n by 2 to remove all even factors
    while n % 2 == 0 {
        max_prime = 2;
        n /= 2;
    }

    // After the above step, n must be odd at this point
    // Start from the smallest odd prime number (3) and keep removing prime factors
    let mut factor = 3;
    while factor * factor <= n {
        while n % factor == 0 {
            max_prime = factor;
            n /= factor;
        }
        factor += 2; // Move to the next odd number
    }

    // If n became a prime number greater than 2 then it's the largest prime factor
    if n > 2 {
        max_prime = n;
    }
    
    max_prime
}

fn main() {
    // Example usage
    let number = 28;
    let largest_factor = largest_prime_factor(number);
    println!("The largest prime factor of {} is {}", number, largest_factor);
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
