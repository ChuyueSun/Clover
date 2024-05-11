
fn get_closest_vowel(word: &str) -> String {
    let vowels = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    let chars: Vec<char> = word.chars().collect();
    
    for w in chars.iter().rev() {
        if !vowels.contains(w) {
            continue;
        }

        if let Some(pos) = word.rfind(*w) {
            if pos > 0 && pos < word.len() - 1 && !vowels.contains(&chars[pos - 1]) && !vowels.contains(&chars[pos + 1]) {
                return w.to_string();
            }
        }
    }

    "".to_string()
}

fn main() {
    let word = "example";
    println!("The closest vowel in '{}' is '{}'", word, get_closest_vowel(word)); // Output should be "a"
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
