
fn check_if_last_char_is_a_letter(txt: &str) -> bool {
    txt.chars().rev().skip_while(char::is_whitespace).next().map_or(false, |c| c.is_alphabetic())
}

fn main() {
    // Test cases
    println!("{}", check_if_last_char_is_a_letter("Hello World!")); // false
    println!("{}", check_if_last_char_is_a_letter("What is that?")); // false
    println!("{}", check_if_last_char_is_a_letter("A lonely island.")); // true
    println!("{}", check_if_last_char_is_a_letter("Hi there ")); // false
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
