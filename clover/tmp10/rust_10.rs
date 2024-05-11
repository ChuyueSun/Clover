
fn is_palindrome(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}

fn shortest_palindrome(s: &str) -> String {
    let rev: String = s.chars().rev().collect();
    for i in 0..s.len() {
        let rest: String = rev[i..].chars().collect();
        if is_palindrome(&rest) {
            let prefix: String = s[..(s.len() - rest.len())].chars().rev().collect();
            return format!("{}{}", prefix, s);
        }
    }
    format!("{}{}", rev, s)
}

fn is_palindrome_10(s: &str) -> String {
    shortest_palindrome(s)
}
`
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
