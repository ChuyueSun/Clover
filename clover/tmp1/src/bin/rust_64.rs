
fn vowels_count(s: &str) -> usize {
    let mut count = 0;
    let chars: Vec<char> = s.chars().collect();
    let last_char = chars.last();

    for &ch in &chars {
        match ch {
            'a' | 'e' | 'i' | 'o' | 'u' => count += 1,
            'y' if Some(&'y') == last_char => count += 1,
            _ => {}
        }
    }

    count
}

fn main() {
    // Example Usage:
    println!("Vowels count: {}", vowels_count("rusty")); // Should print 2
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
