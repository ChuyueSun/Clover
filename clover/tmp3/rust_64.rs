
fn vowels_count(s: &str) -> usize {
    let mut count = 0;
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => count += 1,
            'y' if chars.peek().is_none() => count += 1, // 'y' is a vowel only if it's the last character
            _ => (),
        }
    }
    
    count
}

fn main() {
    // Example usage
    let word = "encyclopedia";
    println!("The word {} has {} vowels", word, vowels_count(word));
}

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_vowels_count() {
        assert!(vowels_count("abcde") == 2);
        assert!(vowels_count("Alone") == 3);
        assert!(vowels_count("key") == 2);
        assert!(vowels_count("bye") == 1);
        assert!(vowels_count("keY") == 2);
        assert!(vowels_count("bYe") == 1);
        assert!(vowels_count("ACEDY") == 3);
    }

}
