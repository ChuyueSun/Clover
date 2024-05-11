
fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as i32) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn is_multiply_prime(mut a: i32) -> bool {
    if a < 1 {
        return false; // Multiplication of primes must be greater than 1
    }

    let mut prime_factors_count = 0;

    for i in 2..100 {
        // Since a is less than 100 and i starts at 2,
        // condition i * i * i <= a ensures we get primes less than 100
        while is_prime(i) && a % i == 0 {
            a /= i;
            prime_factors_count += 1;

            // If the count surpasses 3, it means a is a multiplication of more than 3 primes
            if prime_factors_count > 3 {
                return false;
            }
        }
    }

    // To be a multiplicative of exactly 3 primes, the counter must be 3 and a should be 1.
    prime_factors_count == 3 && a == 1
}

fn main() {
    println!("{}", is_multiply_prime(30)); // true (2 * 3 * 5)
    println!("{}", is_multiply_prime(60)); // false (2 * 2 * 3 * 5)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_multiply_prime() {
        assert!(is_multiply_prime(5) == false);
        assert!(is_multiply_prime(30) == true);
        assert!(is_multiply_prime(8) == true);
        assert!(is_multiply_prime(10) == false);
        assert!(is_multiply_prime(125) == true);
        assert!(is_multiply_prime(3 * 5 * 7) == true);
        assert!(is_multiply_prime(3 * 6 * 7) == false);
        assert!(is_multiply_prime(9 * 9 * 9) == false);
        assert!(is_multiply_prime(11 * 9 * 9) == false);
        assert!(is_multiply_prime(11 * 13 * 7) == true);
    }

}
