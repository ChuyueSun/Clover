
fn is_equal_to_sum_even(n: i32) -> bool {
    if n >= 8 && n % 2 == 0 {
        true
    } else {
        false
    }
}

fn main() {
    // Example usage:
    println!("{}", is_equal_to_sum_even(12)); // true
    println!("{}", is_equal_to_sum_even(7));  // false
    println!("{}", is_equal_to_sum_even(20)); // true
    println!("{}", is_equal_to_sum_even(5));  // false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_equal_to_sum_even() {
        assert_eq!(is_equal_to_sum_even(4), false);
        assert_eq!(is_equal_to_sum_even(6), false);
        assert_eq!(is_equal_to_sum_even(8), true);
        assert_eq!(is_equal_to_sum_even(10), true);
        assert_eq!(is_equal_to_sum_even(11), false);
        assert_eq!(is_equal_to_sum_even(12), true);
        assert_eq!(is_equal_to_sum_even(13), false);
        assert_eq!(is_equal_to_sum_even(16), true);
    }

}
