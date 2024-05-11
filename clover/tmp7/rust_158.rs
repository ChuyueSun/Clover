
fn find_max(words: Vec<&str>) -> String {
    words
        .iter()
        .map(|word| (word.chars().collect::<std::collections::HashSet<_>>().len(), word))
        .max_by(|x, y| {
            x.0.cmp(&y.0).
                then_with(|| y.1.cmp(x.1))
        })
        .map(|(_, word)| word.to_string())
        .unwrap_or_else(String::new)
}

fn main() {
    let words = vec!["apple", "banana", "pear", "orange"];
    println!("{}", find_max(words)); // Should print "banana"
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
