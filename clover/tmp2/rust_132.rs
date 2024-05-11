
fn is_nested(s: &str) -> bool {
    let mut balance = 0;
    let mut was_nested_once = false;

    for c in s.chars() {
        match c {
            '[' => balance += 1,
            ']' if balance > 0 => {
                balance -= 1;
                if balance > 0 {
                    was_nested_once = true;
                }
            }
            _ => return false,
        }
    }
    
    was_nested_once && balance == 0
}

fn main() {
    // Example usage:
    let input = "[[]]";
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
