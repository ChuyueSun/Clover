
fn get_closest_vowel(word: &str) -> String {
    let vowels = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    let word_chars: Vec<_> = word.chars().collect();

    for i in (1..word_chars.len() - 1).rev() {
        if vowels.contains(&word_chars[i])
            && !vowels.contains(&word_chars[i - 1])
            && !vowels.contains(&word_chars[i + 1])
        {
            return word_chars[i].to_string();
        }
    }

    "".to_string()
}

fn main() {
    // Example usage:
    assert_eq!(get_closest_vowel("example"), "a");
    assert_eq!(get_closest_vowel("rhythm"), "");
    assert_eq!(get_closest_vowel("bAce"), "A");

    // Add more tests if needed
    println!("{}", get_closest_vowel("example")); // Outputs: "a"
    println!("{}", get_closest_vowel("rhythm")); // Outputs: ""
    println!("{}", get_closest_vowel("bAce")); // Outputs: "A"
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
