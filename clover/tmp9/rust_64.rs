
fn vowels_count(s: &str) -> usize {
    s.chars()
        .enumerate()
        .filter(|&(i, c)| "aeiou".contains(c) || (c == 'y' && i == s.len() - 1))
        .count()
}

fn main() {
    // Example usage:
    println!("{}", vowels_count("hello")); // Outputs: 2
    println!("{}", vowels_count("sky")); // Outputs: 1
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
