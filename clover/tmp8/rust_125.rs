
fn split_words(txt: &str) -> Vec<String> {
    if txt.contains(' ') {
        txt.split_whitespace()
            .map(|word| word.to_string())
            .collect()
    } else if txt.contains(',') {
        txt.split(',')
            .map(|word| word.to_string())
            .collect()
    } else {
        let odd_count = txt.chars()
            .filter(|&c| c.is_ascii_lowercase() && (c as u32 - 'a' as u32) % 2 == 1)
            .count()
            .to_string();
        vec![odd_count]
    }
}

fn main() {
    // Example usage:
    println!("{:?}", split_words("Hello world!")); // Should split on whitespace
    println!("{:?}", split_words("apple,banana,cherry")); // Should split on commas
    println!("{:?}", split_words("abcdefg")); // Should return the number of lower-case odd order letters
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
