
fn correct_bracketing_parenthesis(bkts: &str) -> bool {
    let mut balance = 0;
    
    for c in bkts.chars() {
        match c {
            '(' => balance += 1,
            ')' => {
                if balance == 0 {
                    return false;
                }
                balance -= 1;
            }
            _ => ()
        }
    }
    
    balance == 0
}

fn main() {
    assert_eq!(correct_bracketing_parenthesis("(()())"), true);
    assert_eq!(correct_bracketing_parenthesis("())"), false);
    assert_eq!(correct_bracketing_parenthesis(")("), false);
    assert_eq!(correct_bracketing_parenthesis("()"), true);
    assert_eq!(correct_bracketing_parenthesis("(()()(()))"), true);
    assert_eq!(correct_bracketing_parenthesis("(())(()"), false);
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
