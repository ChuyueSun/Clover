
fn digits(n: i32) -> i32 {
    let mut n = n;
    let mut product = 1;
    let mut has_odd_digits = false;

    while n > 0 {
        let digit = n % 10;
        if digit % 2 != 0 {
            product *= digit;
            has_odd_digits = true;
        }
        n /= 10;
    }

    if has_odd_digits {
        product
    } else {
        0
    }
}

fn main() {
    // Test cases
    println!("Product of odd digits: {}", digits(12345)); // Should output 15
    println!("Product of odd digits: {}", digits(2468)); // Should output 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digits() {
        assert_eq!(digits(5), 5);
        assert_eq!(digits(54), 5);
        assert_eq!(digits(120), 1);
        assert_eq!(digits(5014), 5);
        assert_eq!(digits(98765), 315);
        assert_eq!(digits(5576543), 2625);
        assert_eq!(digits(2468), 0);
    }

}
