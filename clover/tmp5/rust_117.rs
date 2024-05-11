
fn select_words(s: &str, n: i32) -> Vec<String> {
    if s.is_empty() || n < 0 {
        return Vec::new();
    }

    let consonants = "bcdfghjklmnpqrstvwxyzBCDFGHJKLMNPQRSTVWXYZ";
    s.split_whitespace() // Split the string into words
        .filter_map(|word| {
            let consonant_count = word.chars()
                .filter(|&c| consonants.contains(c))
                .count() as i32;
            
            // Select the word if it has exactly `n` consonants
            if consonant_count == n {
                Some(word.to_string())
            } else {
                None
            }
        })
        .collect()
}

fn main() {
    let s = "This is an example string to test the function";
    let n = 2;
    let selected_words = select_words(s, n);

    println!("Words with exactly {} consonants: {:?}", n, selected_words);
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
