
/// Return a string containing space-delimited numbers starting from 0 up to n inclusive.
///
/// # Arguments
///
/// * `n`: The upper limit of the sequence (inclusive).
///
/// # Returns
///
/// A `String` containing the numbers from 0 to `n`, separated by spaces.
///
/// # Examples
///
/// ```
/// let sequence = string_sequence(5);
/// assert_eq!(sequence, "0 1 2 3 4 5");
/// ```
fn string_sequence(n: i32) -> String {
    (0..=n).map(|x| x.to_string()).collect::<Vec<String>>().join(" ")
}

fn main() {
    // Example usage:
    let result = string_sequence(5);
    println!("{}", result);
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
