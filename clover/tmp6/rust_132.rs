
fn is_nested(input: &str) -> bool {
    let mut count = 0;
    for ch in input.chars() {
        match ch {
            '[' => count += 1,
            ']' if count > 0 => count -= 1,
            ']' => return false,
            _ => return false,
        }
    }
    count == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_nested() {
        assert_eq!(is_nested("[]"), true);
        assert_eq!(is_nested(""), false);
        assert_eq!(is_nested("[][]"), false);
        assert_eq!(is_nested("[[]]"), true);
        assert_eq!(is_nested("[[][]]"), true);
        assert_eq!(is_nested("[[][]][]"), false);
        assert_eq!(is_nested("[[]][]"), true);
        assert_eq!(is_nested("]["), false);
        assert_eq!(is_nested("[[[]"), false);
        assert_eq!(is_nested("]]]"), false);
    }
}

fn main() {
    // Example usage:
    println!("{}", is_nested("[[]]")); // should print true
    println!("{}", is_nested("[][]")); // should print false
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
