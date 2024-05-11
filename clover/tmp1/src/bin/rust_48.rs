
fn is_palindrome(text: String) -> bool {
    let clean_text: String = text.chars().filter(|c| c.is_alphanumeric()).collect::<String>().to_lowercase();
    clean_text.chars().eq(clean_text.chars().rev())
}

fn main() {
    // Examples
    let palindrome = "A man, a plan, a canal: Panama".to_string();
    let not_palindrome = "Hello, world!".to_string();
    println!("Is '{}' a palindrome? {}", palindrome, is_palindrome(palindrome)); // true
    println!("Is '{}' a palindrome? {}", not_palindrome, is_palindrome(not_palindrome)); // false
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
