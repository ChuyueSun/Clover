
fn shortest_palindrome(input: &str) -> String {
    let input_chars: Vec<char> = input.chars().collect();
    let mut max_palindrome_len = 0;

    // Find the longest palindromic suffix
    for i in (0..input.len()).rev() {
        if is_palindrome_10(&input[i..]) {
            max_palindrome_len = input.len() - i;
            break;
        }
    }

    let prefix_to_reverse = &input_chars[0..input.len() - max_palindrome_len];
    let prefix_reversed: String = prefix_to_reverse.iter().rev().collect();

    // Construct the shortest palindrome
    format!("{}{}", prefix_reversed, input)
}

fn is_palindrome_10(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}

fn main() {
    let test_str = "abcd";
    let palindrome = shortest_palindrome(test_str);
    println!("{}", palindrome); // Output will be "dcbabcd"
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
