
fn select_words(s: &str, n: i32) -> Vec<String> {
    if s.is_empty() || n < 0 {
        return Vec::new();
    }

    s.split_whitespace()
        .filter(|word| word.chars().filter(|c| is_consonant(*c)).count() as i32 == n)
        .map(String::from)
        .collect()
}

fn is_consonant(c: char) -> bool {
    match c.to_ascii_lowercase() {
        'a' | 'e' | 'i' | 'o' | 'u' => false,
        _ if c.is_alphabetic() => true,
        _ => false,
    }
}

fn main() {
    let words = select_words("This is a test string with some words", 2);
    println!("{:?}", words);
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
