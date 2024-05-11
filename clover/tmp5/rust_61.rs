
fn correct_bracketing_parenthesis(bkts: &str) -> bool {
    let mut counter = 0;
    
    for c in bkts.chars() {
        match c {
            '(' => counter += 1,
            ')' => {
                if counter == 0 {
                    return false;
                }
                counter -= 1;
            },
            _ => return false, // Invalid character, not a bracket.
        }
    }

    counter == 0
}

fn main() {
    // Example usage:
    let brackets = "(()())";
    println!("Is `{}` correctly bracketed? {}", brackets, correct_bracketing_parenthesis(brackets));
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
