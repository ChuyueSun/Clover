
/// A function that computes the sum of the numbers from 1 to n.
///
/// # Arguments
///
/// * `n`: A 32-bit signed integer representing the upper limit of the sum range.
///
/// # Returns
///
/// The sum of the numbers from 1 to n.
///
/// # Examples
///
/// ```
/// let result = sum_to_n(5);
/// assert_eq!(result, 15); // 1 + 2 + 3 + 4 + 5 = 15
/// ```
fn sum_to_n(n: i32) -> i32 {
    (n * (n + 1)) / 2
}

fn main() {
    // Example usage:
    // This will print "The sum from 1 to 5 is 15"
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
