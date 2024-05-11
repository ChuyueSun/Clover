
fn correct_bracketing_parenthesis(bkts: &str) -> bool {
    let mut counter: i32 = 0;
    
    for c in bkts.chars() {
        match c {
            '(' => counter += 1,
            ')' => counter -= 1,
            _ => {}
        }

        if counter < 0 {
            return false;
        }
    }

    counter == 0
}

fn main() {
    // Example usage:
    let balanced = correct_bracketing_parenthesis("(()())");
    println!("Is '(()())' balanced? {}", balanced);

    let unbalanced = correct_bracketing_parenthesis("(()");
    println!("Is '(()' balanced? {}", unbalanced);
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
