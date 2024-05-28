
fn find_max(words: Vec<&str>) -> &str {
    words
        .iter()
        .max_by(|&&a, &&b| {
            let unique_count_a = a.chars().collect::<std::collections::HashSet<_>>().len();
            let unique_count_b = b.chars().collect::<std::collections::HashSet<_>>().len();
            unique_count_a.cmp(&unique_count_b).then_with(|| a.cmp(&b).reverse())
        })
        .unwrap_or("")
}

fn main() {
    // Example usage:
    let words = vec!["alphabet", "book", "carpet", "cadmium", "framework", "sky"];
    let word_with_max_unique_chars = find_max(words);
    println!("The word with the maximum number of unique characters is: {}", word_with_max_unique_chars);
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