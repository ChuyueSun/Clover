
fn digits(n: i32) -> i32 {
    let mut product = 1;
    let mut no_odd_digits = true;
    let mut current = n;

    while current > 0 {
        let digit = current % 10;
        if digit % 2 != 0 {
            product *= digit;
            no_odd_digits = false;
        }
        current /= 10;
    }

    if no_odd_digits {
        0
    } else {
        product
    }
}

fn main() {
    // Test cases
    println!("Product of odd digits for 12345: {}", digits(12345)); // Should print 15
    println!("Product of odd digits for 2468: {}", digits(2468)); // Should print 0
    println!("Product of odd digits for 13579: {}", digits(13579)); // Should print 945
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
