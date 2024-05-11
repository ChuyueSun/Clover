
fn is_equal_to_sum_even(n: i32) -> bool {
    if n < 8 {
        // The smallest sum of 4 positive even numbers is 2 + 2 + 2 + 2 = 8
        false
    } else {
        // If n is greater than 8 and n is even, it can be represented as a sum of four even numbers
        n % 2 == 0
    }
}

fn main() {
    // Example usage:
    println!("{}", is_equal_to_sum_even(16)); // True, because 16 can be written as 2 + 2 + 6 + 6
    println!("{}", is_equal_to_sum_even(5));  // False, because 5 is odd and less than 8
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
