
fn split_words(txt: &str) -> Vec<String> {
    if txt.contains(' ') {
        txt.split_whitespace().map(|s| s.to_string()).collect()
    } else if txt.contains(',') {
        txt.split(',').map(|s| s.to_string()).collect()
    } else {
        let odd_count = txt
            .chars()
            .filter(|c| c.is_ascii_lowercase() && (c as u32 - 'a' as u32) % 2 == 1)
            .count();
        vec![odd_count.to_string()]
    }
}

fn main() {
    // Example usage
    let words_with_spaces = "Hello World split on spaces";
    let words_with_commas = "Hello,World,split,on,commas";
    let no_spaces_or_commas = "HelloWorld";

    println!("{:?}", split_words(words_with_spaces)); // Should split on spaces
    println!("{:?}", split_words(words_with_commas)); // Should split on commas
    println!("{:?}", split_words(no_spaces_or_commas)); // Should count the odd lowercase letters
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
