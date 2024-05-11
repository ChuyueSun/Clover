
fn is_nested(s: &str) -> bool {
    let mut balance = 0;
    let mut nested = false;

    for char in s.chars() {
        match char {
            '[' => balance += 1,
            ']' => {
                if balance > 1 {
                    nested = true;
                }
                balance -= 1;
                if balance < 0 {
                    return false; // Early return if closing bracket has no matching opening
                }
            }
            _ => return false, // Invalid character, early return
        }
    }

    nested && balance == 0 // Only true if there is a nested sequence and all brackets are balanced
}

fn main() {
    // Example usage:
    println!("{}", is_nested("[[]]")); // true
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
