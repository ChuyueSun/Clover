
fn select_words(s: &str, n: i32) -> Vec<String> {
    if n < 0 {
        return vec![];
    }
    
    let is_consonant = |c: char| match c {
        'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => false,
        _ if c.is_alphabetic() => true,
        _ => false,
    };

    s.split_whitespace()
     .filter(|word| word.chars().filter(is_consonant).count() == n as usize)
     .map(String::from)
     .collect()
}

fn main() {
    // You can test the function like this:
    let words = "This is a Rust programming language";
    let matching_words = select_words(words, 1);
    println!("{:?}", matching_words);
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
