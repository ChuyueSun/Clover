
fn is_equal_to_sum_even(n: i32) -> bool {
    if n < 8 {
        // The smallest sum of 4 positive even numbers is 2 + 2 + 2 + 2 = 8
        return false;
    }

    // Check if the remaining after subtracting the smallest even sum is even
    (n - 8) % 2 == 0
}

fn main() {
    // Example usage:
    println!("{}", is_equal_to_sum_even(8)); // true: 2 + 2 + 2 + 2
    println!("{}", is_equal_to_sum_even(10)); // true: 2 + 2 + 2 + 4
    println!("{}", is_equal_to_sum_even(11)); // false: cannot be written as sum of 4 positive even numbers
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
