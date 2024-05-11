
fn largest_prime_factor(n: i32) -> i32 {
    let mut num = n;
    let mut largest_factor = 2;

    // Divide num by 2 until it is odd
    while num % 2 == 0 {
        num /= 2;
    }

    // Iterate from 3 to the square root of num
    let mut i = 3;
    while i * i <= num {
        // Divide num by i until it is not divisible
        while num % i == 0 {
            num /= i;
            largest_factor = i;
        }
        // Increment the factor by 2, since even numbers cannot be prime (except 2)
        i += 2;
    }

    // If num is a prime number greater than 2
    if num > 2 {
        largest_factor = num;
    }

    largest_factor
}

fn main() {
    let n = 28; // Example usage
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
