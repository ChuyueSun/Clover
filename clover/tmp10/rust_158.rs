
fn find_max(words: Vec<&str>) -> String {
    let mut max_unique_count = 0;
    let mut result_word = "";

    for &word in &words {
        let mut unique_chars = std::collections::HashSet::new();
        for c in word.chars() {
            unique_chars.insert(c);
        }
        let unique_count = unique_chars.len();
        if unique_count > max_unique_count || (unique_count == max_unique_count && word < result_word) {
            max_unique_count = unique_count;
            result_word = word;
        }
    }

    result_word.to_string()
}

fn main() {
    // Example usage
    let words = vec!["apple", "orange", "banana", "grape"];
    let max_unique_word = find_max(words);
    println!("The word with maximum unique characters is: {}", max_unique_word);
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
