
fn correct_bracketing_parenthesis(bkts: &str) -> bool {
    let mut count = 0;

    for c in bkts.chars() {
        match c {
            '(' => count += 1,
            ')' => {
                if count == 0 {
                    return false;
                }
                count -= 1;
            }
            _ => return false, // Invalid character
        }
    }

    count == 0
}

fn main() {
    // Example usage:
    let valid_brackets = "(()())";
    println!("Is '{}' correctly bracketed? {}", valid_brackets, correct_bracketing_parenthesis(valid_brackets));
    
    let invalid_brackets = "(()";
    println!("Is '{}' correctly bracketed? {}", invalid_brackets, correct_bracketing_parenthesis(invalid_brackets));
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
