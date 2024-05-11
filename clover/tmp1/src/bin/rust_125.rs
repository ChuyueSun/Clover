
fn split_words(txt: &str) -> Vec<String> {
    let whitespace_split: Vec<String> = txt.split_whitespace().map(String::from).collect();
    if !whitespace_split.is_empty() && whitespace_split.len() != 1 {
        whitespace_split
    } else {
        let comma_split: Vec<String> = txt.split(',').map(String::from).collect();
        if !comma_split.is_empty() && comma_split.len() != 1 {
            comma_split
        } else {
            let count = txt
                .chars()
                .filter(|c| c.is_ascii_lowercase() && ((c as u8 - b'a') & 1) == 1)
                .count();
            vec![count.to_string()]
        }
    }
}

fn main() {
    // Test cases
    println!("{:?}", split_words("hello world"));       // Output: ["hello", "world"]
    println!("{:?}", split_words("hello,world"));       // Output: ["hello", "world"]
    println!("{:?}", split_words("hello"));             // Output: ["1"]
    println!("{:?}", split_words(""));                  // Output: ["0"]
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
