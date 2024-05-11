
fn is_simple_power(mut x: i32, n: i32) -> bool {
    if x == 1 {
        return true;
    } else if n < 2 {
        return false;
    }
    while x % n == 0 {
        x /= n;
    }
    x == 1
}

fn main() {
    // Example usage:
    println!("{}", is_simple_power(8, 2));  // true, as 2^3 = 8
    println!("{}", is_simple_power(10, 2)); // false, 10 is not a power of 2
}

#[cfg(test)]
mod tests {
    use super::*;

  #[test]
    fn test_is_simple_power() {
        assert!(is_simple_power(1, 4) == true);
        assert!(is_simple_power(2, 2) == true);
        assert!(is_simple_power(8, 2) == true);
        assert!(is_simple_power(3, 2) == false);
        assert!(is_simple_power(3, 1) == false);
        assert!(is_simple_power(5, 3) == false);
        assert!(is_simple_power(16, 2) == true);
        assert!(is_simple_power(143214, 16) == false);
        assert!(is_simple_power(4, 2) == true);
        assert!(is_simple_power(9, 3) == true);
        assert!(is_simple_power(16, 4) == true);
        assert!(is_simple_power(24, 2) == false);
        assert!(is_simple_power(128, 4) == false);
        assert!(is_simple_power(12, 6) == false);
        assert!(is_simple_power(1, 1) == true);
        assert!(is_simple_power(1, 12) == true);
    }

}
