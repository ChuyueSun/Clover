
/// Concatenates a list of strings into a single string.
///
/// # Arguments
///
/// * `strings` - A vector of strings to concatenate.
///
/// # Returns
///
/// A `String` that is the result of concatenating all the strings in the provided vector.
///
/// # Examples
///
/// ```
/// let words = vec!["Hello, ".to_string(), "World!".to_string()];
/// let sentence = concatenate(words);
/// assert_eq!(sentence, "Hello, World!");
/// ```
fn concatenate(strings: Vec<String>) -> String {
    strings.concat()
}

fn main() {
    // Example usage:
    let words = vec!["Hello, ".to_string(), "World!".to_string()];
    let sentence = concatenate(words);
    println!("{}", sentence); // Prints: Hello, World!
}

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_concatenate() {
        assert!(concatenate(vec![]) == "".to_string());
        assert!(
            concatenate(vec!["x".to_string(), "y".to_string(), "z".to_string()])
                == "xyz".to_string()
        );
        assert!(
            concatenate(vec![
                "x".to_string(),
                "y".to_string(),
                "z".to_string(),
                "w".to_string(),
                "k".to_string()
            ]) == "xyzwk".to_string()
        );
    }


}
