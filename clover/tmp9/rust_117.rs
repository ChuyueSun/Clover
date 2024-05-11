
fn select_words(s: &str, n: i32) -> Vec<String> {
    if s.is_empty() || n < 0 {
        return Vec::new();
    }

    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let count_consonants = |word: &str| -> i32 {
        word.chars()
            .filter(|c| !vowels.contains(c) && c.is_alphabetic())
            .count() as i32
    };

    s.split_whitespace()
        .filter(|word| count_consonants(word) == n)
        .map(|word| word.to_string())
        .collect()
}

fn main() {
    let result = select_words("The quick brown fox jumps over the lazy dog", 3);
    println!("{:?}", result);
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
