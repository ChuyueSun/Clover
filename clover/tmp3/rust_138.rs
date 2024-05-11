
fn is_equal_to_sum_even(n: i32) -> bool {
    if n < 8 {
        // The smallest sum of 4 positive even numbers is 2 + 2 + 2 + 2 = 8
        return false;
    }
    
    // If n is even and greater than or equal to 8, it can be expressed as a sum of 4 even numbers
    n % 2 == 0
}

fn main() {
    // Example usage:
    let n = 16;
    println!("Can the number be written as the sum of exactly 4 positive even numbers? {}", is_equal_to_sum_even(n));
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
