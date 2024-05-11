
fn check_if_last_char_is_a_letter(txt: &str) -> bool {
    if let Some(c) = txt.trim_end().chars().last() {
        c.is_alphabetic() && txt.trim_end().ends_with(c)
    } else {
        false
    }
}

fn main() {
    // Example usage:
    let example = "Hello world!";
    println!("{}", check_if_last_char_is_a_letter(example)); // should print true
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
