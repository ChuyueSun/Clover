
fn is_palindrome(s: &str) -> bool {
    s.chars().rev().collect::<String>() == s
}

fn shortest_palindrome(s: &str) -> String {
    if is_palindrome(s) {
        return s.to_string();
    }

    for i in (0..s.len()).rev() {
        let substring = &s[..i];
        if is_palindrome(substring) {
            let prefix = &s[i..s.len()];
            return format!("{}{}", prefix.chars().rev().collect::<String>(), s);
        }
    }

    // This covers the case when no palindromic suffix is found. It appends the reverse of the whole string.
    s.chars().rev().collect::<String>() + s
}

fn main() {
    let input_str = "abcd";
    let palindrome = shortest_palindrome(input_str);
    println!("Shortest palindrome: {}", palindrome);
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
