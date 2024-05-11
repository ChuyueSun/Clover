
fn find_max(words: Vec<&str>) -> Option<String> {
    words
        .iter()
        .map(|word| {
            let unique_chars_count = word.chars().collect::<std::collections::HashSet<_>>().len();
            (unique_chars_count, word)
        })
        .max_by(|a, b| {
            a.0.cmp(&b.0).then_with(|| b.1.cmp(&a.1))
        })
        .map(|(_, word)| word.to_string())
}

fn main() {
    // Example usage:
    let words = vec!["apple", "orange", "banana", "fig", "acerola"];
    match find_max(words) {
        Some(max_word) => println!("Word with maximum unique characters: {}", max_word),
        None => println!("No words given"),
    }
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
