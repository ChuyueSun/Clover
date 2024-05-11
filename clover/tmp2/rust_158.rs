
fn find_max(words: Vec<&str>) -> String {
    words
        .iter()
        .map(|word| (word, word.chars().collect::<std::collections::HashSet<_>>().len()))
        .max_by(|a, b| {
            // Compare by the number of unique characters, then by the string itself
            a.1.cmp(&b.1)
                .reverse() // We want the one with the most unique characters
                .then_with(|| a.0.cmp(b.0)) // In case of tie, sort by lexicographical order
        })
        .map(|(word, _)| word.to_string())
        .unwrap_or_default() // Returns an empty string if the vector is empty
}

fn main() {
    // Example usage:
    let words = vec!["alphabet", "book", "car", "dictionary", "elephant"];
    let max_unique_word = find_max(words);
    println!("The word with the maximum number of unique characters is: {}", max_unique_word);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_max() {
        assert_eq!(find_max(vec!["name", "of", "string"]), "string");
        assert_eq!(find_max(vec!["name", "enam", "game"]), "enam");
        assert_eq!(find_max(vec!["aaaaaaa", "bb", "cc"]), "aaaaaaa");
        assert_eq!(find_max(vec!["abc", "cba"]), "abc");
        assert_eq!(
            find_max(vec!["play", "this", "game", "of", "footbott"]),
            "footbott"
        );
        assert_eq!(find_max(vec!["we", "are", "gonna", "rock"]), "gonna");
        assert_eq!(find_max(vec!["we", "are", "a", "mad", "nation"]), "nation");
        assert_eq!(find_max(vec!["this", "is", "a", "prrk"]), "this");
        assert_eq!(find_max(vec!["b"]), "b");
        assert_eq!(find_max(vec!["play", "play", "play"]), "play");
    }

}
