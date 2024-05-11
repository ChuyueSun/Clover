
fn is_prime(num: i32) -> bool {
    if num <= 1 {
        return false;
    }
    for i in 2..num {
        if num % i == 0 {
            return false;
        }
    }
    true
}

fn is_multiply_prime(a: i32) -> bool {
    if a < 3 {
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
    // Example usage:
    println!("{}", is_multiply_prime(30)); // true: 2 * 3 * 5
    println!("{}", is_multiply_prime(28)); // false: no three prime multiplication equals 28
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
