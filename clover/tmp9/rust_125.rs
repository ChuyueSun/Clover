
fn split_words(txt: &str) -> Vec<String> {
    let words: Vec<String>;
    if txt.contains(' ') {
        words = txt.split_whitespace().map(|s| s.to_string()).collect();
    } else if txt.contains(',') {
        words = txt.split(',').map(|s| s.to_string()).collect();
    } else {
        let odd_lower_count =
            txt.chars()
                .filter(|c| c.is_ascii_lowercase() && (c as u32 - 'a' as u32) % 2 == 1)
                .count();
        words = vec![odd_lower_count.to_string()];
    }
    words
}

fn main() {
    let text = "hello, world";  // Example usage
    println!("{:?}", split_words(text));
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
