
fn check_if_last_char_is_a_letter(txt: &str) -> bool {
    if let Some(last_char) = txt.chars().last() {
        return last_char.is_alphabetic() && txt.chars().rev().skip(1).next().map_or(true, |c| c.is_whitespace());
    }
    false
}

fn main() {
    // Example test cases
    println!("{}", check_if_last_char_is_a_letter("hello.")); // Returns false, '.' is not alphabetic
    println!("{}", check_if_last_char_is_a_letter("hello")); // Returns true, 'o' is alphabetic
    println!("{}", check_if_last_char_is_a_letter("what ")); // Returns false, ' ' is not a character part of the word
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
