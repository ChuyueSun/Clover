
fn is_palindrome(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}

fn make_shortest_palindrome(s: &str) -> String {
    if is_palindrome(s) {
        return s.to_string();
    }

    let mut max_palindrome_size = 0;
    for i in (1..s.len()).rev() {
        if is_palindrome(&s[i..]) {
            max_palindrome_size = s.len() - i;
            break;
        }
    }
    let prefix = &s[..s.len() - max_palindrome_size];
    let add_on = prefix.chars().rev().collect::<String>();

    format!("{}{}", s, add_on)
}

fn main() {
    // Test cases
    let test_str = "race";
    println!("Input: '{}', Shortest palindrome: '{}'", test_str, make_shortest_palindrome(test_str));
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
