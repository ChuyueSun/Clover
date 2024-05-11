
fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..((n as f64).sqrt() as i32 + 1) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn is_multiply_prime(a: i32) -> bool {
    for i in 2..a {
        if a % i == 0 && is_prime(i) {
            let remainder = a / i;
            for j in 2..remainder {
                if remainder % j == 0 && is_prime(j) {
                    let other = remainder / j;
                    if is_prime(other) {
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn main() {
    // Example usage:
    let x = 30; // 2 * 3 * 5
    println!("Is {} a multiple of 3 prime numbers? {}", x, is_multiply_prime(x));
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
