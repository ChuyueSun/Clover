
fn split_words(txt: &str) -> Vec<String> {
    if txt.contains(' ') {
        txt.split_whitespace().map(String::from).collect()
    } else if txt.contains(',') {
        txt.split(',').map(String::from).collect()
    } else {
        let odd_lower_letters_count = txt.chars()
            .filter(|c| c.is_ascii_lowercase() && (c as u32 - 'a' as u32) % 2 == 1)
            .count();
        vec![odd_lower_letters_count.to_string()]
    }
}

fn main() {
    // Example usage:
    let words_space = "a quick brown fox";
    let words_comma = "a,quick,brown,fox";
    let words_none = "abcdef";

    println!("{:?}", split_words(words_space)); // should output ["a", "quick", "brown", "fox"]
    println!("{:?}", split_words(words_comma)); // should output ["a", "quick", "brown", "fox"]
    println!("{:?}", split_words(words_none));  // should output ["3"] because 'b', 'd', 'f' are odd ordered lower-case letters
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_words() {
        assert_eq!(split_words("Hello world!"), vec!["Hello", "world!"]);
        assert_eq!(split_words("Hello,world!"), vec!["Hello", "world!"]);
        assert_eq!(split_words("Hello world,!"), vec!["Hello", "world,!"]);
        assert_eq!(
            split_words("Hello,Hello,world !"),
            vec!["Hello,Hello,world", "!"]
        );
        assert_eq!(split_words("abcdef"), vec!["3"]);
        assert_eq!(split_words("aaabb"), vec!["2"]);
        assert_eq!(split_words("aaaBb"), vec!["1"]);
        assert_eq!(split_words(""), vec!["0"]);
    }

}
