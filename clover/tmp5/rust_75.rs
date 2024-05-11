
fn is_prime(num: i32) -> bool {
    if num <= 1 {
        return false;
    }
    if num <= 3 {
        return true;
    }
    if num % 2 == 0 || num % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= num {
        if num % i == 0 || num % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn is_multiply_prime(a: i32) -> bool {
    for x in 2..100 {
        if a % x == 0 && is_prime(x) {
            let yz = a / x;
            for y in 2..yz {
                if yz % y == 0 && is_prime(y) {
                    let z = yz / y;
                    if is_prime(z) {
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn main() {
    // You can test the function with some value here
    println!("Is 30 the multiplication of 3 prime numbers? {}", is_multiply_prime(30));
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
