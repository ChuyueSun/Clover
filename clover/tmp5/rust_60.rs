
/// Sums numbers from 1 to n.
///
/// # Arguments
///
/// * `n` - The upper bound to sum to (inclusive).
///
/// # Returns
///
/// An i32 representing the sum of all numbers from 1 to n.
///
/// # Examples
///
/// ```
/// let sum = sum_to_n(5);
/// assert_eq!(sum, 15); // 1 + 2 + 3 + 4 + 5
/// ```
fn sum_to_n(n: i32) -> i32 {
    (1..=n).sum()
}

fn main() {
    let sum = sum_to_n(5);
    println!("The sum from 1 to 5 is {}", sum);
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