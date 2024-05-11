
fn is_simple_power(mut x: i32, n: i32) -> bool {
    if n <= 1 {
        return x == n;
    }
    while x % n == 0 {
        x /= n;
    }
    x == 1
}

fn main() {
    // You can test the function with some examples here.
    println!("Is 8 a simple power of 2? {}", is_simple_power(8, 2)); // true
    println!("Is 10 a simple power of 2? {}", is_simple_power(10, 2)); // false
    println!("Is 1 a simple power of 3? {}", is_simple_power(1, 3)); // true as 3^0 = 1
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
