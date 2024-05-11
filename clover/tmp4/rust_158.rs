
fn find_max(words: Vec<&str>) -> &str {
    let mut max_unique_chars = 0;
    let mut result = "";

    for word in words.iter() {
        let mut chars = word.chars().collect::<Vec<char>>();
        chars.sort_unstable();
        chars.dedup();

        let unique_char_count = chars.len();

        if unique_char_count > max_unique_chars || 
          (unique_char_count == max_unique_chars && word < &result) {
            max_unique_chars = unique_char_count;
            result = word;
        }
    }

    result
}

fn main() {
    let words = vec!["alphabet", "book", "carpet", "cadmium", "framework", "specter", "spectre"];
    let word = find_max(words);
    println!("The word with the most unique characters is: {}", word);
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
