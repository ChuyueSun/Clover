
fn is_nested(input: &str) -> bool {
    let mut balance = 0;
    let mut had_nested = false;

    for c in input.chars() {
        match c {
            '[' => balance += 1,
            ']' if balance > 0 => {
                balance -= 1;
                had_nested = true;
            }
            _ => (),
        }
    }

    had_nested && balance == 0
}

fn main() {
    assert_eq!(is_nested("[]"), false);
    assert_eq!(is_nested("[[]]"), true);
    assert_eq!(is_nested("[][]"), false);
    assert_eq!(is_nested("[[][]]"), true);
    assert_eq!(is_nested("[[[]]]"), true);
    assert_eq!(is_nested("][["), false);
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
