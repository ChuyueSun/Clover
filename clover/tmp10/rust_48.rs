
fn is_palindrome(text: String) -> bool {
    let filtered_text: String = text.chars().filter(|c| c.is_alphanumeric()).collect();
    let reversed_text: String = filtered_text.chars().rev().collect();
    filtered_text.eq_ignore_ascii_case(&reversed_text)
}

fn main() {
    let test_str = "A man, a plan, a canal, Panama".to_string();
    println!("Is '{}' a palindrome? {}", test_str, is_palindrome(test_str));
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
