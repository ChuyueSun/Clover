
fn vowels_count(s: &str) -> usize {
    let mut count = 0;
    for (i, c) in s.chars().enumerate() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => count += 1,
            'y' if i == s.len() - 1 => count += 1,
            _ => {}
        }
    }
    count
}

fn main() {
    // Example usage
    let word = "honey";
    println!("Vowels in {}: {}", word, vowels_count(word));
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
