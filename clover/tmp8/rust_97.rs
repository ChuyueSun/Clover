
fn multiply(a: i32, b: i32) -> i32 {
    let unit_digit_a = a % 10;
    let unit_digit_b = b % 10;
    unit_digit_a * unit_digit_b
}

fn main() {
    // Example usage:
    let result = multiply(123, 456);
    println!("The product of the unit digits is: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply() {
        assert!(multiply(148, 412) == 16);
        assert!(multiply(19, 28) == 72);
        assert!(multiply(2020, 1851) == 0);
        assert!(multiply(14, -15) == 20);
        assert!(multiply(76, 67) == 42);
        assert!(multiply(17, 27) == 49);
        assert!(multiply(0, 1) == 0);
        assert!(multiply(0, 0) == 0);
    }

}
