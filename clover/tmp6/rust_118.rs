
fn get_closest_vowel(word: &str) -> String {
    let vowels = "aeiouAEIOU";
    let mut consonants = "bcdfghjklmnpqrstvwxyzBCDFGHJKLMNPQRSTVWXYZ".chars().collect::<HashSet<_>>();

    // Reverse the characters in the word to search from the right
    let reversed_chars: Vec<char> = word.chars().rev().collect();
    let mut iterator = reversed_chars.iter().peekable();

    while let Some(&ch) = iterator.next() {
        if consonants.contains(&ch) {
            // We found a consonant, now let's see if there's a vowel next
            if let Some(&&next_ch) = iterator.peek() {
                if vowels.contains(next_ch) {
                    // There's a vowel, now let's check if it's between two consonants
                    iterator.next(); // Skip the vowel we just found
                    if let Some(&&next_next_ch) = iterator.peek() {
                        if consonants.contains(next_next_ch) {
                            // We found the pattern consonant-vowel-consonant
                            return next_ch.to_string();
                        }
                    }
                }
            }
        }
    }

    String::new()
}

fn main() {
    // Example usage:
    let word = "example";
    let closest_vowel = get_closest_vowel(word);
    println!("Closest vowel found: {}", closest_vowel); // Should print "e"
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
