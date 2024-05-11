
fn select_words(s: &str, n: i32) -> Vec<String> {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    s.split_whitespace()
        .filter_map(|word| {
            let consonants_count = word.chars().filter(|&c| !vowels.contains(&c)).count() as i32;
            if consonants_count == n {
                Some(word.to_string())
            } else {
                None
            }
        })
        .collect()
}

fn main() {
    let words = select_words("This is a Test string with some words", 2);
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
