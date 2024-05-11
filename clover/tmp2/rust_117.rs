
fn select_words(s: &str, n: i32) -> Vec<String> {
    let is_consonant = |c: char| c.is_ascii_alphabetic() && !"aeiouAEIOU".contains(c);

    s.split_whitespace()
        .filter_map(|word| {
            if word.chars().filter(|&c| is_consonant(c)).count() == n as usize {
                Some(word.to_string())
            } else {
                None
            }
        })
        .collect()
}

fn main() {
    // Example usage:
    let sentence = "The quick brown fox jumps over the lazy dog";
    let consonant_count = 3;
    let words_with_consonants = select_words(sentence, consonant_count);
    println!("{:?}", words_with_consonants);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_words() {
        assert_eq!(select_words("Mary had a little lamb", 4), vec!["little"]);
        assert_eq!(
            select_words("Mary had a little lamb", 3),
            vec!["Mary", "lamb"]
        );
        let v_empty: Vec<&str> = vec![];
        assert_eq!(select_words("simple white space", 2), v_empty);
        assert_eq!(select_words("Hello world", 4), vec!["world"]);
        assert_eq!(select_words("Uncle sam", 3), vec!["Uncle"]);
        assert_eq!(select_words("", 4), v_empty);
        assert_eq!(select_words("a b c d e f", 1), vec!["b", "c", "d", "f"]);
    }

}
