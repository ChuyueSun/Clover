
fn is_palindrome(text: String) -> bool {
    let text_lower = text.to_lowercase().chars().filter(|c| c.is_alphanumeric()).collect::<String>();
    text_lower == text_lower.chars().rev().collect::<String>()
}

fn main() {
    let test_str = "A man, a plan, a canal, Panama".to_string();
    println!("Is the string '{}' a palindrome? {}", test_str, is_palindrome(test_str));
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
