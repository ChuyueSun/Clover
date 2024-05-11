
fn is_prime(num: i32) -> bool {
    match num {
        2 | 3 => true,
        _ if num <= 1 || num % 2 == 0 => false,
        _ => {
            let mut divisor = 3;
            while divisor * divisor <= num {
                if num % divisor == 0 {
                    return false;
                }
                divisor += 2;
            }
            true
        }
    }
}

fn is_multiply_prime(a: i32) -> bool {
    if a < 2 * 2 * 2 {
        return false;
    }

    for i in 2..100 {
        if is_prime(i) {
            for j in 2..100 {
                if is_prime(j) {
                    for k in 2..100 {
                        if is_prime(k) && i * j * k == a {
                            return true;
                        }
                    }
                }
            }
        }
    }
    false
}

fn main() {
    // Example calls for the `is_multiply_prime` function
    println!("Is 30 multiply of 3 prime numbers? {}", is_multiply_prime(30)); // Two possible sets are (2, 3, 5) or (3, 5, 2)
    println!("Is 100 multiply of 3 prime numbers? {}", is_multiply_prime(100)); // No sets of three primes multiply to 100
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
