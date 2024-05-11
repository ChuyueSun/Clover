
use std::collections::HashSet;

/// Check if two words have the same characters.
/// 
/// This function checks if `str1` and `str2` consist of the exact same set of characters,
/// ignoring any duplicates, and regardless of the order. It returns true if `str1` and `str2`
/// have the same characters, otherwise it returns false.
///
/// # Arguments
///
/// * `str1` - A string slice that holds the first word to compare.
/// * `str2` - A string slice that holds the second word to compare.
///
/// # Examples
///
/// ```
/// let word1 = "listen";
/// let word2 = "silent";
/// assert_eq!(same_chars(word1, word2), true); // These words have the same characters
///
/// let word1 = "triangle";
/// let word2 = "integral";
/// assert_eq!(same_chars(word1, word2), true); // These words have the same characters
///
/// let word1 = "hello";
/// let word2 = "world";
/// assert_eq!(same_chars(word1, word2), false); // These words do not have the same characters
/// ```
fn same_chars(str1: &str, str2: &str) -> bool {
    let set1: HashSet<char> = str1.chars().collect();
    let set2: HashSet<char> = str2.chars().collect();
    set1 == set2
}

fn main() {
    // Example usage:
    let word1 = "listen";
    let word2 = "silent";
    println!("Do '{}' and '{}' have the same characters? {}", word1, word2, same_chars(word1, word2));

    let word1 = "triangle";
    let word2 = "integral";
    println!("Do '{}' and '{}' have the same characters? {}", word1, word2, same_chars(word1, word2));

    let word1 = "hello";
    let word2 = "world";
    println!("Do '{}' and '{}' have the same characters? {}", word1, word2, same_chars(word1, word2));
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_same_chars() {
        assert!(same_chars("eabcdzzzz", "dddzzzzzzzddeddabc") == true);
        assert!(same_chars("abcd", "dddddddabc") == true);
        assert!(same_chars("dddddddabc", "abcd") == true);
        assert!(same_chars("eabcd", "dddddddabc") == false);
        assert!(same_chars("abcd", "dddddddabcf") == false);
        assert!(same_chars("eabcdzzzz", "dddzzzzzzzddddabc") == false);
        assert!(same_chars("aabb", "aaccc") == false);
    }

}
