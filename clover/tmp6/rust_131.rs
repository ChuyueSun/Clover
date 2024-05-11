
fn product_of_odd_digits(n: i32) -> i32 {
    let mut n = n;
    let mut product = 1;
    let mut has_odd = false;

    while n > 0 {
        let digit = n % 10;
        if digit % 2 != 0 {
            product *= digit;
            has_odd = true;
        }
        n /= 10;
    }

    if has_odd {
        product
    } else {
        0
    }
}

fn main() {
    // Test cases
    println!("Product of odd digits in 123456: {}", product_of_odd_digits(123456)); // Output: 15
    println!("Product of odd digits in 2468: {}", product_of_odd_digits(2468)); // Output: 0
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
