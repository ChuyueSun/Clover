
fn get_closest_vowel(word: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    let chars: Vec<char> = word.chars().collect();

    for i in (1..chars.len() - 1).rev() {
        if !vowels.contains(&chars[i - 1])
            && vowels.contains(&chars[i])
            && !vowels.contains(&chars[i + 1])
        {
            return chars[i].to_string();
        }
    }
    "".to_string()
}

fn main() {
    // Example usage:
    let word = "planetarium";
    println!("Closest vowel: {}", get_closest_vowel(word)); // Should print "a"
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
