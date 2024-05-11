
fn is_palindrome_10(s: &str) -> String {
    let s_chars: Vec<char> = s.chars().collect();
    let mut suffix = s_chars.len();

    while suffix > 0 {
        let substring: String = s_chars[s_chars.len() - suffix..].iter().collect();
        if substring == substring.chars().rev().collect::<String>() {
            break;
        }
        suffix -= 1;
    }

    let prefix: String = s_chars[..s_chars.len() - suffix].iter().rev().collect();
    format!("{}{}", s, prefix)
}

fn main() {
    let input = "race";
    let shortest_palindrome = is_palindrome_10(input);
    println!("{}", shortest_palindrome); // Should output: "racecar"
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
