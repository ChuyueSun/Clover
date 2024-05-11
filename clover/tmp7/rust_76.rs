
fn is_simple_power(x: i32, n: i32) -> bool {
    if n == 0 {
        return x == 1;
    }
    if n == 1 {
        return true;
    }

    let mut current_power = n;
    while current_power <= x && current_power > 0 {
        if current_power == x {
            return true;
        }
        current_power = match current_power.checked_mul(n) {
            Some(p) => p,
            None => break,
        };
    }
    false
}

fn main() {
    println!("{}", is_simple_power(1, 5)); // true, because 5^0 = 1
    println!("{}", is_simple_power(8, 2)); // true, because 2^3 = 8
    println!("{}", is_simple_power(10, 2)); // false, because no power of 2 equals 10
    println!("{}", is_simple_power(81, 3)); // true, because 3^4 = 81
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
