
fn largest_prime_factor(n: i32) -> i32 {
    let mut number = n;
    let mut largest_factor = 2;

    while number % 2 == 0 {
        number /= 2;
    }

    let mut factor = 3;
    while factor * factor <= number {
        while number % factor == 0 {
            largest_factor = factor;
            number /= factor;
        }
        factor += 2;
    }

    if number > 2 {
        largest_factor = number;
    }

    largest_factor
}

fn main() {
    // Example use
    let n = 60;
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
