
fn is_palindrome(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}

fn shortest_palindrome(s: &str) -> String {
    if is_palindrome(s) {
        return s.to_string();
    }

    let mut prefix_len = 0;
    for i in (0..s.len()).rev() {
        let prefix = &s[..i];
        if is_palindrome(prefix) {
            prefix_len = prefix.len();
            break;
        }
    }

    let suffix = &s[prefix_len..];
    let reversed_suffix: String = suffix.chars().rev().collect();
    format!("{}{}", reversed_suffix, s)
}

fn main() {
    let input_string = "abcd";
    let palindrome = shortest_palindrome(input_string);
    println!("{}", palindrome); // Should print "dcbabcd"
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
