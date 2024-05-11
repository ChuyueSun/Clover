
fn is_palindrome(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}

fn shortest_palindrome(str: &str) -> String {
    let len = str.len();
    let s = str.chars().collect::<Vec<_>>();
    let mut suffix_len = len;

    while suffix_len > 0 {
        if is_palindrome(&s[..suffix_len].iter().collect::<String>()) {
            break;
        }
        suffix_len -= 1;
    }

    s[..(len - suffix_len)]
        .iter()
        .rev()
        .chain(s.iter())
        .collect::<String>()
}

fn main() {
    let str = "abcd"; // Replace "abcd" with any string to test
    let pal = shortest_palindrome(str);
    println!("Shortest palindrome: {}", pal);
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
