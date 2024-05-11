
fn is_palindrome(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}

fn shortest_palindrome(s: &str) -> String {
    let mut prefix_rev = String::new();
    for ch in s.chars() {
        prefix_rev.insert(0, ch);
        let candidate = format!("{}{}", s, prefix_rev);
        if is_palindrome(&candidate) {
            return candidate;
        }
    }
    s.to_string()
}

fn main() {
    // Example usage:
    let original = "race";
    let palindrome = shortest_palindrome(original);
    println!("Shortest palindrome: {}", palindrome);
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
