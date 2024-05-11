
fn get_closest_vowel(word: &str) -> String {
    let vowels = "aeiouAEIOU";
    let mut found_vowel = None;

    let chars: Vec<char> = word.chars().collect();
    
    for (i, &ch) in chars.iter().enumerate().rev() {
        if vowels.contains(ch) {
            if i > 0 && i < chars.len() - 1 {
                if !vowels.contains(chars[i - 1]) && !vowels.contains(chars[i + 1]) {
                    found_vowel = Some(ch);
                    break;
                }
            }
        }
    }
    
    found_vowel.map(|v| v.to_string()).unwrap_or_else(|| "".to_string())
}

fn main() {
    // You can use this for testing, e.g.
    // println!("{}", get_closest_vowel("example")); // should print "a"
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
