
/// Returns the length of the given string.
/// 
/// # Arguments
///
/// * `input` - A `String` whose length will be measured.
///
/// # Returns
///
/// The length of the input `String`.
/// 
/// # Examples
///
/// ```
/// let len = strlen(String::from("Hello, world!"));
/// assert_eq!(len, 13);
/// ```
fn strlen(input: String) -> usize {
    input.len()
}

fn main() {
    let example_string = String::from("Hello, world!");
    println!("The length of '{}' is {}.", example_string, strlen(example_string));
}

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_strlen() {
        assert!(strlen("".to_string()) == 0);
        assert!(strlen("x".to_string()) == 1);
        assert!(strlen("asdasnakj".to_string()) == 9);
    }

}
