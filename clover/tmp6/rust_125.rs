
fn split_words(txt: &str) -> Vec<String> {
    if txt.contains(' ') {
        txt.split_whitespace().map(String::from).collect()
    } else if txt.contains(',') {
        txt.split(',').map(String::from).collect()
    } else {
        let odd_count = txt.chars()
            .filter(|c| c.is_ascii_lowercase() && (c as usize - 'a' as usize) % 2 == 1)
            .count();
        vec![odd_count.to_string()]
    }
}

fn main() {
    // Test Examples
    let test1 = "The quick brown fox";
    println!("{:?}", split_words(test1)); // Should split on whitespaces, so it will return vector of words.

    let test2 = "The,quick,brown,fox";
    println!("{:?}", split_words(test2)); // Should split on commas, so it'll return vector of words.

    let test3 = "abcd";
    println!("{:?}", split_words(test3)); // The letters 'b' and 'd' have odd orders, so it will return ["2"].
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
