
fn vowels_count(s: &str) -> usize {
    s.chars()
        .enumerate()
        .filter(|&(i, c)| {
            "aeiou".contains(c) || (i == s.chars().count() - 1 && c == 'y')
        })
        .count()
}

fn main() {
    // Example usage
    let word = "encyclopedia";
    println!("Number of vowels: {}", vowels_count(word));
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
