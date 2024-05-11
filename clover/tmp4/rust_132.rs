
fn is_nested(s: &str) -> bool {
    let mut bracket_stack = Vec::new();

    for c in s.chars() {
        match c {
            '[' => bracket_stack.push(c),
            ']' => {
                if bracket_stack.is_empty() {
                    // If stack is empty, no corresponding opening bracket
                    return false;
                }
                bracket_stack.pop();
            }
            _ => {
                // If the string contains characters other than brackets, return false
                return false;
            }
        }
    }

    !bracket_stack.is_empty()
}

fn main() {
    // Example Usage
    // Should return true since there is a valid nested subsequence: "[[]]"
    println!("{}", is_nested("[[]]"));
    // Should return false since all brackets are at the same level and not nested: "[]"
    println!("{}", is_nested("[]"));
    // Should return false since the brackets are not balanced: "[[]["
    println!("{}", is_nested("[[]["));
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
