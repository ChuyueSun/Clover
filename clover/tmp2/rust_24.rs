
fn largest_divisor(n: i32) -> i32 {
    if n <= 1 {
        return 0;
    }
    for i in (2..n).rev() {
        if n % i == 0 {
            return i;
        }
    }
    1
}

fn main() {
    // Test cases
    println!("Largest divisor of 10 is {}", largest_divisor(10)); // Should print 5
    println!("Largest divisor of 17 is {}", largest_divisor(17)); // Should print 1, since 17 is a prime number
    println!("Largest divisor of 28 is {}", largest_divisor(28)); // Should print 14
}

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_largest_divisor() {
        assert!(largest_divisor(3) == 1);
        assert!(largest_divisor(7) == 1);
        assert!(largest_divisor(10) == 5);
        assert!(largest_divisor(100) == 50);
        assert!(largest_divisor(49) == 7);
    }

}
