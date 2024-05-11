
fn get_closest_vowel(word: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    let mut chars_iter = word.chars().rev();

    while let Some(c) = chars_iter.next() {
        if !vowels.contains(&c) {
            if let Some(vowel) = chars_iter.next() {
                if vowels.contains(&vowel) {
                    if let Some(next_c) = chars_iter.next() {
                        if !vowels.contains(&next_c) {
                            return vowel.to_string();
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
    println!("{}", get_closest_vowel("hello")); // Should print "e"
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
