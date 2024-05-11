
fn is_palindrome(text: String) -> bool {
    // Remove any non-alphanumeric characters and convert to lowercase to ensure consistency
    let cleaned_text: String = text
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_lowercase().to_string())
        .collect();
    
    // Compare the cleaned text to its reverse
    cleaned_text == cleaned_text.chars().rev().collect::<String>()
}

fn main() {
    // Test cases
    println!("{}", is_palindrome(String::from("A man, a plan, a canal: Panama"))); // Should return true
    println!("{}", is_palindrome(String::from("race a car"))); // Should return false
    println!("{}", is_palindrome(String::from(" "))); // Should return true
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
