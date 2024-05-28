
fn digits(n: i32) -> i32 {
    let mut n = n;
    let mut product = 1;
    let mut has_odd_digit = false;
    
    while n > 0 {
        let digit = n % 10;
        if digit % 2 != 0 {
            product *= digit;
            has_odd_digit = true;
        }
        n /= 10;
    }
    
    if has_odd_digit {
        product
    } else {
        0
    }
}

fn main() {
    // Example usage:
    println!("Product of odd digits: {}", digits(123456)); // Output: 15
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