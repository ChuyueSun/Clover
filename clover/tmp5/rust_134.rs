
fn check_if_last_char_is_a_letter(txt: &str) -> bool {
    let trimmed_txt = txt.trim_end();
    trimmed_txt.ends_with(|c: char| c.is_alphabetic() && trimmed_txt.chars().nth_back(1).map_or(true, |b| b.is_whitespace()))
}

fn main() {
    // Example usage
    println!("{}", check_if_last_char_is_a_letter("hello ")); // Should print true
    println!("{}", check_if_last_char_is_a_letter("world")); // Should print false
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
