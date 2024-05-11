
fn is_nested(input: &str) -> bool {
    let mut stack = Vec::new();

    for c in input.chars() {
        match c {
            '[' => stack.push(c),
            ']' => {
                if stack.pop().is_none() {
                    return false;
                } else if stack.is_empty() {
                    // At least one ']' encountered which means previous '[' had a matching closing
                    // bracket and hence was nested.
                    return true;
                }
            },
            _ => return false, // Invalid character, string should only contain square brackets
        }
    }

    false // No nested brackets found or invalid string
}

fn main() {
    // You can call the function here to test
    // Example:
    // println!("{}", is_nested("[][]")); // Should print false
    // println!("{}", is_nested("[[]]")); // Should print true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_nested() {
        assert_eq!(is_nested("[[]]"), true);
        assert_eq!(is_nested("[]]]]]]][[[[[]"), false);
        assert_eq!(is_nested("[][]"), false);
        assert_eq!(is_nested("[]"), false);
        assert_eq!(is_nested("[[[[]]]]"), true);
        assert_eq!(is_nested("[]]]]]]]]]]"), false);
        assert_eq!(is_nested("[][][[]]"), true);
        assert_eq!(is_nested("[[]"), false);
        assert_eq!(is_nested("[]]"), false);
        assert_eq!(is_nested("[[]][["), true);
        assert_eq!(is_nested("[[][]]"), true);
        assert_eq!(is_nested(""), false);
        assert_eq!(is_nested("[[[[[[[["), false);
        assert_eq!(is_nested("]]]]]]]]"), false);
    }

}
