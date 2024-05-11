
fn is_nested(s: &str) -> bool {
    let mut balance = 0;
    let mut has_nested = false;

    for c in s.chars() {
        if c == '[' {
            balance += 1;
        } else if c == ']' {
            if balance > 1 {
                has_nested = true;
            }
            balance -= 1;
            if balance < 0 {
                return false;  // String is invalid if closing brackets exceed opening.
            }
        }
    }

    has_nested && balance == 0
}

fn main() {
    // Test cases
    println!("{}", is_nested("[]")); // false
    println!("{}", is_nested("[[]]")); // true
    println!("{}", is_nested("[][]")); // false
    println!("{}", is_nested("[[][]]")); // true
    println!("{}", is_nested("[[[]]]")); // true
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
