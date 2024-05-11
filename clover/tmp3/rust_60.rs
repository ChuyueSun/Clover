
/// sum_to_n is a function that sums numbers from 1 to n.
///
/// Parameters:
/// - `n`: The upper bound of the range to sum up.
///
/// Returns:
/// The sum of all numbers from 1 to `n` inclusive.
fn sum_to_n(n: i32) -> i32 {
    (1..=n).fold(0, |sum, item| sum + item)
}

fn main() {
    // Example: summing numbers from 1 to 5
    let result = sum_to_n(5);
    println!("The sum is: {}", result);
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
