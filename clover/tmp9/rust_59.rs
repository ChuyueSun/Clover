
fn largest_prime_factor(n: i32) -> i32 {
    let mut num = n;
    let mut largest_factor = 2;

    while num % 2 == 0 {
        num /= 2;
    }

    let sqrt_n = (num as f64).sqrt() as i32;
    let mut factor = 3;
    while factor <= sqrt_n {
        while num % factor == 0 {
            largest_factor = factor;
            num /= factor;
        }
        factor += 2;
    }

    if num > 2 {
        largest_factor = num;
    }

    largest_factor
}

fn main() {
    let n = 13195;
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
