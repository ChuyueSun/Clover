
fn largest_prime_factor(mut n: i32) -> i32 {
    let mut largest_factor: i32 = 1;
    let mut p: i32 = 2;

    while p * p <= n {
        while n % p == 0 {
            largest_factor = p;
            n /= p;
        }
        p += 1;
    }

    if n > 1 {
        largest_factor = n;
    }

    largest_factor
}

fn main() {
    // Example usage:
    let n = 28; // 28 = 2 * 2 * 7
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
