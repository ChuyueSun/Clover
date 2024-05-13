
fn main() {

}

fn make_palindrome(s: &str) -> String {
    let mut result = s.to_string();
    let mut prefix = String::new();
    for i in 0..s.len() {
        let suffix = &s[i..];
        if is_palindrome(suffix) {
            prefix = s[..i].chars().rev().collect();
            break;
        }
    }
    result.push_str(&prefix);
    result
}

fn is_palindrome(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
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
