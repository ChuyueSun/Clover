
fn correct_bracketing_parenthesis(bkts: &str) -> bool {
    let mut counter = 0;
    for c in bkts.chars() {
        match c {
            '(' => counter += 1,
            ')' => counter -= 1,
            _ => {} // Assume the input is valid containing only '(' and ')'
        }
        // If counter is negative, there are more closing than opening brackets
        if counter < 0 {
            return false;
        }
    }
    // True if all brackets are correctly closed
    counter == 0
}

fn main() {
    // Example usage:
    // Correct bracketing
    assert_eq!(correct_bracketing_parenthesis("()()"), true);
    // Incorrect bracketing
    assert_eq!(correct_bracketing_parenthesis("(()"), false);
    // Incorrect bracketing
    assert_eq!(correct_bracketing_parenthesis(")("), false);
    
    // Feel free to add more examples and test cases
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
