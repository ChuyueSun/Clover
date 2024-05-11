
fn multiply(a: i32, b: i32) -> i32 {
    (a % 10) * (b % 10)
}

fn main() {
    // Example usage:
    println!("{}", multiply(123, 456)); // Output: 18 because 3 * 6 = 18
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
