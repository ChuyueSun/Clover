
fn split_words(txt: &str) -> Vec<String> {
    let ws_split = txt.split_whitespace().map(String::from).collect::<Vec<String>>();

    if ws_split.len() > 1 {
        return ws_split;
    }

    let comma_split = txt.split(',').map(String::from).collect::<Vec<String>>();

    if comma_split.len() > 1 {
        return comma_split;
    }

    let odd_lower_case_count = txt.chars()
        .filter(|c| c.is_ascii_lowercase() && (c as u8 - b'a') % 2 == 1)
        .count();

    vec![odd_lower_case_count.to_string()]
}

fn main() {
    // Example usages:
    println!("{:?}", split_words("The quick brown fox")); // Split by whitespaces
    println!("{:?}", split_words("The,quick,brown,fox")); // Split by commas
    println!("{:?}", split_words("Thequickbrownfox")); // Number of lowercase letters with odd order
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
