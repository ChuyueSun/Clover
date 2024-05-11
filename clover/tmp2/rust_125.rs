
fn split_words(txt: &str) -> Vec<String> {
    if txt.contains(' ') {
        txt.split_whitespace().map(|s| s.to_string()).collect()
    } else if txt.contains(',') {
        txt.split(',').map(|s| s.to_string()).collect()
    } else {
        let odd_lowercase_count = txt.chars()
            .filter(|c| c.is_ascii_lowercase() && (*c as u8 - b'a') % 2 == 1)
            .count();
        vec![odd_lowercase_count.to_string()]
    }
}

fn main() {
    // Example usage:
    let sentence = "Given a string of words";
    println!("{:?}", split_words(sentence));
    let no_spaces = "split,on,commas";
    println!("{:?}", split_words(no_spaces));
    let no_commans_or_spaces = "abcdefghijk";
    println!("{:?}", split_words(no_commans_or_spaces));
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
