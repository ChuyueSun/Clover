
fn is_nested(s: &str) -> bool {
    let mut count = 0;
    for c in s.chars() {
        if c == '[' {
            count += 1;
        } else if c == ']' {
            count -= 1;
        }

        if count < 0 {
            return false; // If the count goes negative, it means there are unmatched closing brackets
        }
    }
    
    // To have a nested subsequence, there should be at least two consecutive '[' or ']'
    // A valid string with nested brackets should have had the count increased and decreased,
    // hence count must end up zero and we should have seen count > 1 at some point.
    count == 0 && s.contains("[[") || s.contains("]]")
}

fn main() {
    // Example usage:
    // This example has a valid nested subsequence "[[]]"
    println!("{}", is_nested("[[]]")); // Outputs: true
    
    // This example does not have any nested subsequences
    println!("{}", is_nested("[][]")); // Outputs: false
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
