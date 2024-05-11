
/// Return a string containing space-delimited numbers starting from 0 up to n inclusive.
///
/// # Arguments
///
/// * `n` - An i32 integer specifying the end of the sequence, inclusive.
///
/// # Examples
///
/// ```
/// let sequence = string_sequence(5);
/// assert_eq!(sequence, "0 1 2 3 4 5");
/// ```
fn string_sequence(n: i32) -> String {
    (0..=n).map(|i| i.to_string()).collect::<Vec<_>>().join(" ")
}

fn main() {
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
