
fn digits(n: i32) -> i32 {
    let mut num = n;
    let mut product = 1;
    let mut has_odd_digit = false;

    while num > 0 {
        let digit = num % 10;
        if digit % 2 != 0 {
            product *= digit;
            has_odd_digit = true;
        }
        num /= 10;
    }

    if has_odd_digit {
        product
    } else {
        0
    }
}

fn main() {
    // Example usage:
    println!("{}", digits(123456)); // Outputs 15, which is the product of 1, 3, and 5
    println!("{}", digits(2468));    // Outputs 0, since all digits are even
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
