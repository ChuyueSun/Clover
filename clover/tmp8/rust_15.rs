
/// Generate a string containing space-delimited numbers starting from 0 up to n inclusive.
///
/// # Arguments
///
/// * `n`: The upper bound integer of the sequence, inclusive.
///
/// # Returns
///
/// A `String` that contains the sequence of numbers starting from 0 up to and including `n`,
/// each number separated by a space. If `n` is negative, returns an empty string.
///
/// # Examples
///
/// ```
/// assert_eq!(string_sequence(5), "0 1 2 3 4 5");
/// assert_eq!(string_sequence(0), "0");
/// assert_eq!(string_sequence(-1), "");
/// ```
fn string_sequence(n: i32) -> String {
    if n < 0 {
        return String::new();
    }
    
    (0..=n).map(|i| i.to_string()).collect::<Vec<String>>().join(" ")
}

fn main() {
    // Example usage:
    println!("{}", string_sequence(5)); // Outputs: "0 1 2 3 4 5"
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
