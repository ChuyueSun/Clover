
fn get_closest_vowel(word: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    let mut found_consonant = false;

    for &ch in word.chars().rev() {
        if !vowels.contains(&ch) {
            found_consonant = true;
        } else if found_consonant {
            // Check if the next character in reverse (previous in original order) is a consonant
            if let Some(next_char) = word.chars().rev().skip_while(|&x| x != ch).nth(1) {
                if !vowels.contains(&next_char) {
                    return ch.to_string();
                }
            }
            found_consonant = false; // Reset to handle consecutive vowels
        }
    }

    "".to_string()
}

fn main() {
    // Test cases
    println!("{}", get_closest_vowel("hello")); // Output: "e"
    println!("{}", get_closest_vowel("racecar")); // Output: "a"
    println!("{}", get_closest_vowel("RUST")); // Output: ""
    println!("{}", get_closest_vowel("starte")); // Output: ""
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
