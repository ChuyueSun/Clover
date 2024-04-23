fn main() {}

/*

    Create a function that returns True if the last character
    of a given string is an alphabetical character and is not
    a part of a word, and False otherwise.
    Note: "word" is a group of characters separated by space.

*/

fn check_if_last_char_is_a_letter(txt: &str) -> bool {
    if txt.len() == 0 {
        return false;
    }
    let chr = txt.chars().last().unwrap();
    if chr < 'A' || (chr > 'Z' && chr < 'a') || chr > 'z' {
        return false;
    }
    if txt.len() == 1 {
        return true;
    }
    let chr = txt.chars().nth(txt.len() - 2).unwrap();
    if (chr >= 'A' && chr <= 'Z') || (chr >= 'a' && chr <= 'z') {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_if_last_char_is_a_letter() {
        assert_eq!(check_if_last_char_is_a_letter("apple"), false);
        assert_eq!(check_if_last_char_is_a_letter("apple pi e"), true);
        assert_eq!(check_if_last_char_is_a_letter("eeeee"), false);
        assert_eq!(check_if_last_char_is_a_letter("A"), true);
        assert_eq!(check_if_last_char_is_a_letter("Pumpkin pie "), false);
        assert_eq!(check_if_last_char_is_a_letter("Pumpkin pie 1"), false);
        assert_eq!(check_if_last_char_is_a_letter(""), false);
        assert_eq!(check_if_last_char_is_a_letter("eeeee e "), false);
        assert_eq!(check_if_last_char_is_a_letter("apple pie"), false);
    }
}
