
fn is_multiply_prime(a: i32) -> bool {
    if a < 3 {
        // Since 1 and 2 are not products of three primes, return false.
        return false;
    }

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
        let mut i: i32 = 5;
        while i * i <= n {
            if n % i == 0 || n % (i + 2) == 0 {
                return false;
            }
            i += 6;
        }
        true
    }

    // Iterate through numbers to check if a is a product of three primes.
    for i in 2..100 {
        if is_prime(i) {
            let mut j = i;
            while j < a / i {
                if is_prime(j) && a % (i * j) == 0 {
                    let k = a / (i * j);
                    if is_prime(k) {
                        return true;
                    }
                }
                j += 1;
            }
        }
    }
    false
}

fn main() {
    // Example usage:
    println!("Is 30 a multiplication of 3 prime numbers? {}", is_multiply_prime(30)); // Should output: true
    println!("Is 100 a multiplication of 3 prime numbers? {}", is_multiply_prime(100)); // Should output: false
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
