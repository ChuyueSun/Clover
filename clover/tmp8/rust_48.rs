
fn is_palindrome(text: String) -> bool {
    let processed: String = text.chars().filter(|c| c.is_alphanumeric()).collect::<String>().to_lowercase();
    processed.chars().zip(processed.chars().rev()).all(|(a, b)| a == b)
}

fn main() {
    let example = String::from("A man, a plan, a canal: Panama");
    println!("Is '{}' a palindrome? {}", example, is_palindrome(example));
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
