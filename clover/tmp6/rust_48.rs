
fn is_palindrome(text: String) -> bool {
    let forwards = text.chars();
    let backwards = text.chars().rev();
    forwards.eq(backwards)
}

fn main() {
    // Example usage:
    // Should print true as "racecar" is a palindrome
    println!("{}", is_palindrome("racecar".to_string()));

    // Should print false as "rust" is not a palindrome
    println!("{}", is_palindrome("rust".to_string()));
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome("".to_string()) == true);
        assert!(is_palindrome("aba".to_string()) == true);
        assert!(is_palindrome("aaaaa".to_string()) == true);
        assert!(is_palindrome("zbcd".to_string()) == false);
        assert!(is_palindrome("xywyx".to_string()) == true);
        assert!(is_palindrome("xywyz".to_string()) == false);
        assert!(is_palindrome("xywzx".to_string()) == false);
    }

}
