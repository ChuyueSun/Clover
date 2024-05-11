
fn check_if_last_char_is_a_letter(txt: &str) -> bool {
    if let Some(last_char) = txt.chars().rev().find(|c| !c.is_whitespace()) {
        return last_char.is_alphabetic();
    }
    false
}

fn main() {
    // Test cases
    println!("{}", check_if_last_char_is_a_letter("Hello ")); // true
    println!("{}", check_if_last_char_is_a_letter("World!")); // false
    println!("{}", check_if_last_char_is_a_letter("How are you today? ")); // true
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
