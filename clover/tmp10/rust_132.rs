
fn is_nested(input: &str) -> bool {
    let mut depth = 0;

    for c in input.chars() {
        match c {
            '[' => depth += 1,
            ']' => {
                if depth > 0 {
                    depth -= 1;
                } else {
                    // Early return if there exists a closing bracket without a matching opening bracket
                    return false;
                }
            }
            _ => return false, // Invalid character
        }
    }

    // A valid nested subsequence requires a depth increase followed by a decrease.
    depth == 0 // If depth is not zero, some opening brackets do not have matching closing brackets.
}

fn main() {
    // Example usage:
    let input = "[][]";
    println!("Is nested: {}", is_nested(input));
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
