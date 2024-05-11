
fn is_simple_power(x: i32, n: i32) -> bool {
    if n == 0 {
        return x == 1;
    }
    if x > 0 && n > 0 {
        let mut power = n;
        while power <= x {
            if power == x {
                return true;
            }
            match power.checked_mul(n) {
                Some(next_power) => power = next_power,
                None => return false,
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_simple_power() {
        assert_eq!(is_simple_power(8, 2), true);
        assert_eq!(is_simple_power(9, 3), true);
        assert_eq!(is_simple_power(16, 4), true);
        assert_eq!(is_simple_power(13, 2), false);
        assert_eq!(is_simple_power(0, 0), false);
        assert_eq!(is_simple_power(1, 0), true);
        assert_eq!(is_simple_power(-27, 3), false);
        assert_eq!(is_simple_power(27, -3), false);
    }
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
