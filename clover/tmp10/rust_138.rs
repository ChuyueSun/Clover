
fn is_equal_to_sum_even(n: i32) -> bool {
    if n < 8 { // The smallest sum of 4 even numbers is 2 + 2 + 2 + 2 = 8
        return false;
    }
    
    n % 2 == 0 // If n is even, it can always be written as the sum of 4 even numbers
}

fn main() {
    // Example usage:
    println!("{}", is_equal_to_sum_even(8)); // true
    println!("{}", is_equal_to_sum_even(10)); // true
    println!("{}", is_equal_to_sum_even(15)); // false
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
