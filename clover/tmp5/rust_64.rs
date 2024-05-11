
fn vowels_count(s: &str) -> usize {
    let mut count = 0;
    let chars: Vec<char> = s.chars().collect();
    let length = chars.len();

    for (i, c) in chars.iter().enumerate() {
        // Check for regular vowels in any position
        if "aeiou".contains(*c) {
            count += 1;
        }
        // Check for 'y' being the last character in the string
        else if *c == 'y' && i == length - 1 {
            count += 1;
        }
    }

    count
}

fn main() {
    // Example usage:
    let word = "rhythm";
    println!("The word '{}' has {} vowels", word, vowels_count(word));
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
