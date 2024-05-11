
fn correct_bracketing_parenthesis(bkts: &str) -> bool {
    let mut balance = 0;

    for bracket in bkts.chars() {
        match bracket {
            '(' => balance += 1,
            ')' => {
                if balance == 0 {
                    return false;
                }
                balance -= 1;
            }
            _ => continue,
        }
    }

    balance == 0
}

fn main() {
    // Example usage:
    println!("{}", correct_bracketing_parenthesis("(()())")); // expected output: true
    println!("{}", correct_bracketing_parenthesis("(()")); // expected output: false
    println!("{}", correct_bracketing_parenthesis(")(")); // expected output: false
    println!("{}", correct_bracketing_parenthesis("(())")); // expected output: true
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
