
fn split_words(txt: &str) -> Vec<String> {
    let whitespaces = txt.contains(' ');
    let commas = txt.contains(',');

    if whitespaces {
        txt.split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    } else if commas {
        txt.split(',')
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>()
    } else {
        let odd_lower_letters: String = txt.chars()
            .filter(|&c| c.is_ascii_lowercase() && (c as usize - 'a' as usize) % 2 == 1)
            .collect();
        vec![odd_lower_letters.len().to_string()]
    }
}

fn main() {
    // Example usage:
    println!("{:?}", split_words("This is a test")); // split on whitespace
    println!("{:?}", split_words("One,Two,Three")); // split on commas
    println!("{:?}", split_words("abcdefg")); // count odd lowercase letters
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
