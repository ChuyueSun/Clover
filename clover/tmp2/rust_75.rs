
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
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn is_multiply_prime(a: i32) -> bool {
    if a < 2 {
        return false;
    }

    let mut count = 0;
    let mut number = a;

    for i in 2..100 {
        while is_prime(i) && number % i == 0 {
            count += 1;
            number /= i;
            if count > 3 {
                return false;
            }
        }
    }

    count == 3 && number == 1
}

fn main() {
    // Example to test the function
    println!("{}", is_multiply_prime(30)); // should print true since 30 = 2 * 3 * 5
    println!("{}", is_multiply_prime(100)); // should print false as 100 is not a product of exactly three primes
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
