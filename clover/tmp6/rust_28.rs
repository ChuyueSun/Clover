
/// Concatenates a list of strings into a single string.
///
/// # Arguments
/// * `strings` - A vector of strings to be concatenated.
///
/// # Returns
/// A `String` that is the result of concatenating all the strings in the vector.
///
/// # Examples
/// ```
/// let words = vec![String::from("Hello"), String::from(" "), String::from("world!")]
/// let result = concatenate(words);
/// assert_eq!(result, "Hello world!");
/// ```
fn concatenate(strings: Vec<String>) -> String {
    strings.join("")
}

fn main() {
    // Example usage:
    let words = vec![String::from("Hello"), String::from(" "), String::from("world!")];
    let result = concatenate(words);
    println!("{}", result); // This will print: Hello world!
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
