
fn is_nested(s: &str) -> bool {
    let mut balance = 0;
    let mut nested = false;

    for c in s.chars() {
        if c == '[' {
            balance += 1;
        } else if c == ']' {
            if balance > 1 {
                nested = true;
            }
            balance -= 1;
            if balance < 0 {
                return false;
            }
        } else {
            // Since input should only contain square brackets, return false if any other char is found
            return false;
        }
    }

    nested && balance == 0
}

fn main() {
    // Example usage:
    // `is_nested` should return true for this string as it contains a valid nested subsequence "[[]]"
    println!("{}", is_nested("[[]]"));
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
