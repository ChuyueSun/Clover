
fn correct_bracketing_parenthesis(bkts: &str) -> bool {
    let mut balance = 0;
    for ch in bkts.chars() {
        match ch {
            '(' => balance += 1,
            ')' => {
                if balance == 0 {
                    return false;
                }
                balance -= 1;
            }
            _ => {} // Ignore other characters if any
        }
    }
    balance == 0
}

fn main() {
    // Test cases
    assert_eq!(correct_bracketing_parenthesis("()"), true);
    assert_eq!(correct_bracketing_parenthesis("(()())"), true);
    assert_eq!(correct_bracketing_parenthesis(")("), false);
    assert_eq!(correct_bracketing_parenthesis("((((()))"), false);
    println!("All tests passed!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correct_bracketing_parenthesis() {
        assert!(correct_bracketing_parenthesis("()"));
        assert!(correct_bracketing_parenthesis("(()())"));
        assert!(correct_bracketing_parenthesis("()()(()())()"));
        assert!(correct_bracketing_parenthesis("()()((()()())())(()()(()))"));
        assert!(!(correct_bracketing_parenthesis("((()())))")));
        assert!(!(correct_bracketing_parenthesis(")(()")));
        assert!(!(correct_bracketing_parenthesis("(")));
        assert!(!(correct_bracketing_parenthesis("((((")));
        assert!(!(correct_bracketing_parenthesis(")")));
        assert!(!(correct_bracketing_parenthesis("(()")));
        assert!(!(correct_bracketing_parenthesis("()()(()())())(()")));
        assert!(!(correct_bracketing_parenthesis("()()(()())()))()")));
    }

}
