
fn vowels_count(s: &str) -> usize {
    let mut count = 0;
    let chars: Vec<char> = s.chars().collect();
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    for (i, &c) in chars.iter().enumerate() {
        if vowels.contains(&c.to_lowercase().next().unwrap()) {
            count += 1;
        } else if c == 'y' && i == chars.len() - 1 {
            count += 1;
        }
    }

    count
}

fn main() {
    // Example usage
    let word = "encyclopedia";
    println!("The number of vowels in '{}' is {}", word, vowels_count(word));
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
