
/// Evaluates whether the given number `n` can be written as the sum of exactly 4 positive even numbers.
///
/// # Arguments
///
/// * `n` - The integer to be evaluated.
///
/// # Returns
///
/// * `bool` - `true` if `n` can be written as the sum of exactly 4 positive even numbers;
///            `false` otherwise.
fn is_equal_to_sum_even(n: i32) -> bool {
    if n < 8 {
        // The smallest sum of 4 positive even numbers is 2 + 2 + 2 + 2 = 8
        return false;
    }
    // If `n` is even and greater than or equal to 8, it can always be written as the sum
    // of four even numbers because we can use 2, 2, 2, and (n - 6) to form `n`
    // If `n` is odd or lesser than 8, it cannot be written as the sum of even numbers
    n % 2 == 0
}

fn main() {
    // Example usage:
    let result = is_equal_to_sum_even(20);
    println!("Can be written as the sum of exactly 4 even numbers: {}", result);
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
