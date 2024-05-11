
fn select_words(s: &str, n: i32) -> Vec<String> {
    if s.is_empty() {
        return vec![];
    }

    let is_consonant = |c: char| c.is_alphabetic() && !"aeiouAEIOU".contains(c);

    s.split_whitespace()
        .filter(|&word| word.chars().filter(is_consonant).count() as i32 == n)
        .map(String::from)
        .collect()
}

fn main() {
    let test_str = "This is a test string with some words";
    let consonant_count = 2;
    let selected_words = select_words(test_str, consonant_count);
    println!("{:?}", selected_words); // This should print words with exactly 2 consonants.
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
