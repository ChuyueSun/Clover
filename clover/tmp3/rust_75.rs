
fn is_multiply_prime(a: i32) -> bool {
    fn is_prime(n: i32) -> bool {
        if n <= 1 {
            false
        } else if n <= 3 {
            true
        } else if n % 2 == 0 || n % 3 == 0 {
            false
        } else {
            let mut i = 5;
            while i * i <= n {
                if n % i == 0 || n % (i + 2) == 0 {
                    return false;
                }
                i += 6;
            }
            true
        }
    }

    for i in 2..100 {
        if a % i == 0 && is_prime(i) {
            let mut current_val = a / i;
            for j in 2..i {
                if current_val % j == 0 && is_prime(j) {
                    let k = current_val / j;
                    if j < k && is_prime(k) {
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn main() {
    // Example usage
    println!("Is 30 multiply by 3 prime numbers? {}", is_multiply_prime(30)); // Should print true
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
