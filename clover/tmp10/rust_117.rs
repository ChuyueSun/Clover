
fn select_words(s: &str, n: i32) -> Vec<String> {
    if s.is_empty() || n < 0 {
        return Vec::new();
    }

    s.split_whitespace()
        .filter_map(|word| {
            let consonant_count = word.chars().filter(|&c| {
                matches!(c.to_ascii_lowercase(), 'b' | 'c' | 'd' | 'f' | 'g' | 'h' | 'j' | 'k' | 'l' | 'm' | 'n' | 'p' | 'q' | 'r' | 's' | 't' | 'v' | 'w' | 'x' | 'y' | 'z')
            }).count() as i32;
            if consonant_count == n {
                Some(word.to_string())
            } else {
                None
            }
        })
        .collect()
}

fn main() {
    let s = "Hello world this is a test string";
    let n = 3;
    let words_with_n_consonants = select_words(s, n);
    println!("{:?}", words_with_n_consonants);
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
