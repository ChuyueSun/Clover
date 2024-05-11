
fn check_if_last_char_is_a_letter(txt: &str) -> bool {
    if let Some(ch) = txt.chars().rev().find(|c| !c.is_whitespace()) {
        ch.is_alphabetic() && !txt.ends_with(ch)
    } else {
        false
    }
}

fn main() {
    // Example usage:
    let example1 = "Hello, world!";
    println!("{} => {}", example1, check_if_last_char_is_a_letter(example1)); // True

    let example2 = "Hello, world";
    println!("{} => {}", example2, check_if_last_char_is_a_letter(example2)); // False

    let example3 = "";
    println!("{} => {}", example3, check_if_last_char_is_a_letter(example3)); // False

    let example4 = "   ";
    println!("{} => {}", example4, check_if_last_char_is_a_letter(example4)); // False
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
