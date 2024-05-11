
/// Sums numbers from 1 to n (inclusive).
/// 
/// # Arguments
/// * `n`: An integer to which the numbers will be summed (n >= 1).
/// 
/// # Returns
/// An integer representing the sum of all numbers from 1 to `n`.
///
/// # Examples
/// ```
/// let result = sum_to_n(5);
/// assert_eq!(result, 15); // 1 + 2 + 3 + 4 + 5
/// ```
fn sum_to_n(n: i32) -> i32 {
    (1..=n).fold(0, |acc, x| acc + x)
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
