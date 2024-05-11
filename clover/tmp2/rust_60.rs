
/// Sums numbers from 1 to n.
///
/// # Arguments
///
/// * `n` - The upper bound of the range to sum.
///
/// # Returns
///
/// An integer representing the sum of all numbers from 1 to `n`.
///
/// # Examples
///
/// ```
/// let result = sum_to_n(5);
/// assert_eq!(result, 15);
/// ```
fn sum_to_n(n: i32) -> i32 {
    (1..=n).fold(0, |sum, item| sum + item)
}

fn main() {
    // Example usage:
    let result = sum_to_n(5);
    println!("The sum from 1 to 5 is: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_to_n() {
        assert!(sum_to_n(1) == 1);
        assert!(sum_to_n(6) == 21);
        assert!(sum_to_n(11) == 66);
        assert!(sum_to_n(30) == 465);
        assert!(sum_to_n(100) == 5050);
    }

}
