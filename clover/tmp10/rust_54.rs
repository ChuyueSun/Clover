
use std::collections::HashSet;

/// Check if two words have the same characters.
///
/// # Arguments
///
/// * `str1`: A string slice that holds the first word.
/// * `str2`: A string slice that holds the second word.
///
/// # Returns
///
/// This function returns a boolean value:
/// - `true` if both words contain exactly the same characters.
/// - `false` otherwise.
fn same_chars(str1: &str, str2: &str) -> bool {
    let set1: HashSet<char> = str1.chars().collect();
    let set2: HashSet<char> = str2.chars().collect();
    set1 == set2
}

fn main() {
    // Example usage:
    let word1 = "listen";
    let word2 = "silent";
    println!("Do the words '{}' and '{}' have the same characters? {}", word1, word2, same_chars(word1, word2));
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
