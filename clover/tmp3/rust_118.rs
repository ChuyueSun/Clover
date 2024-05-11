
fn get_closest_vowel(word: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    let mut chars = word.chars().rev();
    let mut prev_consonant = false;
    
    while let Some(c) = chars.next() {
        if prev_consonant && vowels.contains(&c) {
            if let Some(next_c) = chars.next() {
                if !vowels.contains(&next_c) {
                    return c.to_string();
                }
            }
        }
        prev_consonant = !vowels.contains(&c);
    }
    
    "".to_string()
}

fn main() {
    // Example usage:
    let word = "programmer";
    println!("The closest vowel: {}", get_closest_vowel(word));
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
