
fn largest_prime_factor(mut n: i32) -> i32 {
    let mut largest_factor: i32 = 1;
    let mut i: i32 = 2;

    while i * i <= n {
        while n % i == 0 {
            largest_factor = i;
            n /= i;
        }
        i += 1;
    }

    if n > 1 {
        largest_factor = n;
    }

    largest_factor
}

fn main() {
    // Test the function with a composite number.
    let number: i32 = 24; // The prime factors of 24 are 2, 3 and the largest is 3.
    println!("The largest prime factor of {} is {}", number, largest_prime_factor(number));
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
