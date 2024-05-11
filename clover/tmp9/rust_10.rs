
fn shortest_palindrome(s: &str) -> String {
    let rev_s: String = s.chars().rev().collect();
    let combined = s.to_string() + &rev_s;
    for i in 0..s.len() {
        if combined[i..].starts_with(s) {
            return combined[i..].to_string();
        }
    }
    combined
}

fn main() {
    let input = "abc";
    let palindrome = shortest_palindrome(input);
    println!("The shortest palindrome starting with '{}' is '{}'", input, palindrome);
}

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_make_palindrome() {
        assert_eq!(make_palindrome(""), "");
        assert_eq!(make_palindrome("x"), "x");
        assert_eq!(make_palindrome("xyz"), "xyzyx");
        assert_eq!(make_palindrome("xyx"), "xyx");
        assert_eq!(make_palindrome("jerry"), "jerryrrej");
    }

}
