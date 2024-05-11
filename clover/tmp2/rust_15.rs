
/// Generates a space-delimited string of numbers from 0 to `n` inclusive.
///
/// # Arguments
///
/// * `n` - The last number in the sequence.
///
/// # Returns
///
/// A `String` that contains the space-delimited numbers from 0 to `n`.
///
/// # Examples
///
/// ```
/// let sequence = string_sequence(5);
/// assert_eq!(sequence, "0 1 2 3 4 5");
/// ```
fn string_sequence(n: i32) -> String {
    (0..=n).map(|i| i.to_string()).collect::<Vec<String>>().join(" ")
}

fn main() {
    // Example usage:
    let sequence = string_sequence(5);
    println!("{}", sequence);
}

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_string_sequence() {
        assert!(string_sequence(0) == "0".to_string());
        assert!(string_sequence(3) == "0 1 2 3".to_string());
        assert!(string_sequence(10) == "0 1 2 3 4 5 6 7 8 9 10".to_string());
    }

}
