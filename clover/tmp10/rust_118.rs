
fn get_closest_vowel(word: &str) -> String {
    let vowels = "aeiouAEIOU";
    let mut found_vowel = None;

    let chars: Vec<char> = word.chars().collect();
    let len = chars.len();

    for i in (1..len - 1).rev() {
        if vowels.contains(chars[i])
            && !vowels.contains(chars[i - 1])
            && !vowels.contains(chars[i + 1])
        {
            found_vowel = Some(chars[i]);
            break;
        }
    }
    found_vowel.map_or_else(String::new, |v| v.to_string())
}

fn main() {
    println!("{}", get_closest_vowel("hello"));       // Outputs "e"
    println!("{}", get_closest_vowel("environment")); // Outputs "o"
    println!("{}", get_closest_vowel("rhythm"));      // Outputs ""
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
