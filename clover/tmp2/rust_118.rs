
fn get_closest_vowel(word: &str) -> String {
    let vowels = "aeiouAEIOU";
    
    for (i, ch) in word.chars().enumerate().rev() {
        // Ensure we are not at the start or end of the word
        if i > 0 && i < word.len() - 1 {
            if !vowels.contains(ch) {
                // Check if characters around are vowels
                let prev_char = word.chars().nth(i - 1).unwrap();
                let next_char = word.chars().nth(i + 1).unwrap();

                if vowels.contains(prev_char) && !vowels.contains(next_char) {
                    return prev_char.to_string();
                }
            }
        }
    }

    "".to_string()
}

fn main() {
    // Test cases
    println!("{}", get_closest_vowel("example")); // should return "a"
    println!("{}", get_closest_vowel("Rust")); // should return "u"
    println!("{}", get_closest_vowel("rhythm")); // should return ""
    println!("{}", get_closest_vowel("bOOkkeeper")); // should return "e"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_closest_vowel() {
        assert_eq!(get_closest_vowel("yogurt"), "u");
        assert_eq!(get_closest_vowel("full"), "u");
        assert_eq!(get_closest_vowel("easy"), "");
        assert_eq!(get_closest_vowel("eAsy"), "");
        assert_eq!(get_closest_vowel("ali"), "");
        assert_eq!(get_closest_vowel("bad"), "a");
        assert_eq!(get_closest_vowel("most"), "o");
        assert_eq!(get_closest_vowel("ab"), "");
        assert_eq!(get_closest_vowel("ba"), "");
        assert_eq!(get_closest_vowel("quick"), "");
        assert_eq!(get_closest_vowel("anime"), "i");
        assert_eq!(get_closest_vowel("Asia"), "");
        assert_eq!(get_closest_vowel("Above"), "o");
    }

}
