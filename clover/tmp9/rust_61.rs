
fn correct_bracketing_parenthesis(bkts: &str) -> bool {
    let mut counter = 0;

    for c in bkts.chars() {
        if c == '(' {
            counter += 1;
        } else if c == ')' {
            if counter == 0 {
                return false;
            }
            counter -= 1;
        }
    }

    counter == 0
}

fn main() {
    // Example usage
    let brackets1 = "((()))";
    let brackets2 = "(()";
    let brackets3 = ")(";
    
    assert_eq!(correct_bracketing_parenthesis(brackets1), true);
    assert_eq!(correct_bracketing_parenthesis(brackets2), false);
    assert_eq!(correct_bracketing_parenthesis(brackets3), false);
    
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
