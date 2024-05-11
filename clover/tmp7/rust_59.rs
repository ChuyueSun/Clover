
fn is_prime(x: i32) -> bool {
    if x <= 1 {
        return false;
    }
    for i in 2..=((x as f32).sqrt() as i32) {
        if x % i == 0 {
            return false;
        }
    }
    true
}

fn largest_prime_factor(mut n: i32) -> i32 {
    let mut max_prime = -1;

    // Divide n by 2 to remove all even factors
    while n % 2 == 0 {
        max_prime = 2;
        n /= 2;
    }

    // n must be odd at this point, so a skip of 2 (i += 2) can be used.
    for i in (3..).step_by(2).take_while(|&i| i * i <= n) {
        while n % i == 0 {
            max_prime = i;
            n /= i;
        }
    }

    // This condition is to handle the case when n is a prime number
    // greater than 2.
    if n > 2 && is_prime(n) {
        max_prime = n;
    }

    max_prime
}

fn main() {
    // Example Usage:
    let n = 28; // Not a prime number, and greater than 1
    println!("The largest prime factor of {} is {}", n, largest_prime_factor(n));
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
