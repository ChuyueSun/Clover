
fn is_equal_to_sum_even(n: i32) -> bool {
    if n < 8 {
        // Smallest sum of 4 even numbers is 2 + 2 + 2 + 2 = 8.
        return false;
    }

    // As the sum of 4 even numbers is always an even number,
    // n must be even to be written as the sum of 4 even numbers.
    n % 4 == 0
}

fn main() {
    // Examples
    println!("{}", is_equal_to_sum_even(12)); // Should print true
    println!("{}", is_equal_to_sum_even(14)); // Should print false
    println!("{}", is_equal_to_sum_even(16)); // Should print true
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
