
fn select_words(s: &str, n: i32) -> Vec<String> {
    if s.is_empty() || n < 0 {
        return Vec::new();
    }
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    s.split_whitespace().filter(|word| {
        word.chars().filter(|&c| !vowels.contains(&c.to_lowercase().next().unwrap())).count() == n as usize
    }).map(|word| word.to_string()).collect()
}

fn main() {
    let test_string = "This is a test string with some words";
    let n_consonants = 3;
    let words_with_n_consonants = select_words(test_string, n_consonants);
    println!("Words with {} consonants: {:?}", n_consonants, words_with_n_consonants);
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
