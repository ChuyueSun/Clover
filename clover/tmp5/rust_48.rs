
fn is_palindrome(text: String) -> bool {
    let cleaned_text: String = text.chars().filter(|c| c.is_alphanumeric()).collect::<String>().to_lowercase();
    cleaned_text == cleaned_text.chars().rev().collect::<String>()
}

fn main() {
    println!("{}", is_palindrome(String::from("A man, a plan, a canal, Panama"))); // true
    println!("{}", is_palindrome(String::from("racecar"))); // true
    println!("{}", is_palindrome(String::from("hello"))); // false
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
