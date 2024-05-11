
fn is_simple_power(x: i32, n: i32) -> bool {
    if n <= 0 {
        return false;
    }
    
    let mut num = x;
    while num > 1 {
        if num % n != 0 {
            return false;
        }
        num /= n;
    }
    num == 1
}

fn main() {
    // Example usage:
    println!("{}", is_simple_power(8, 2)); // true, because 2**3 = 8
    println!("{}", is_simple_power(9, 3)); // true, because 3**2 = 9
    println!("{}", is_simple_power(14, 3)); // false, because 3**n != 14 for any integer n
    println!("{}", is_simple_power(0, 5)); // false, because n**int cannot be 0 if n > 0
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
