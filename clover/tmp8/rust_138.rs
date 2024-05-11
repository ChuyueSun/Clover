
fn is_equal_to_sum_even(n: i32) -> bool {
    if n < 8 {
        // If n is less than 8, it can't be the sum of 4 positive even numbers.
        return false;
    }

    // Check if n - 8 (4 smallest even numbers) is even.
    // If so, then n can be expressed as a sum of 4 even numbers because
    // n = (n - 8) + 2 + 2 + 2 + 2 (where 2 + 2 + 2 + 2 represents the 4 smallest even numbers)
    (n - 8) % 2 == 0
}

fn main() {
    // Example usage:
    println!("{}", is_equal_to_sum_even(28));  // true
    println!("{}", is_equal_to_sum_even(7));   // false
    println!("{}", is_equal_to_sum_even(14));  // true
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
