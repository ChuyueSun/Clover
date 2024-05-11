
fn split_words(txt: &str) -> Vec<String> {
    let whitespace_split: Vec<String> = txt.split_whitespace().map(|s| s.to_string()).collect();
    if !whitespace_split.is_empty() && whitespace_split.len() > 1 {
        return whitespace_split;
    }
    
    let comma_split: Vec<String> = txt.split(',').map(|s| s.to_string()).collect();
    if !comma_split.is_empty() && comma_split.len() > 1 {
        return comma_split;
    }

    // Count the number of lower-case letters with odd order in the alphabet
    let odd_order_count = txt.chars()
        .filter(|c| c.is_ascii_lowercase())
        .filter(|c| (c as u32 - 'a' as u32) % 2 == 1)
        .count();
    
    vec![odd_order_count.to_string()]
}

fn main() {
    // Example usage:
    println!("{:?}", split_words("This is a test")); // Should split on whitespaces
    println!("{:?}", split_words("No,whitespaces,here")); // Should split on commas
    println!("{:?}", split_words("abcdefg")); // Should return number of lower-case letters with odd order
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
