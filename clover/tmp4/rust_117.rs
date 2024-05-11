
fn is_consonant(c: char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => false,
        'A' | 'E' | 'I' | 'O' | 'U' => false,
        _ if c.is_alphabetic() => true,
        _ => false,
    }
}

fn select_words(s: &str, n: i32) -> Vec<String> {
    if n < 0 {
        return vec![];
    }
    
    s.split_whitespace()
        .filter_map(|word| {
            let consonant_count = word.chars().filter(|&c| is_consonant(c)).count();
            if consonant_count == n as usize {
                Some(word.to_string())
            } else {
                None
            }
        })
        .collect()
}

fn main() {
    let example = "This is an example string with several words";
    let words_with_2_consonants = select_words(example, 2);
    println!("{:?}", words_with_2_consonants);
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
