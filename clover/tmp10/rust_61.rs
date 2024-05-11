
fn correct_bracketing_parenthesis(bkts: &str) -> bool {
    let mut balance = 0;
    for bracket in bkts.chars() {
        match bracket {
            '(' => balance += 1,
            ')' => {
                balance -= 1;
                if balance < 0 {
                    return false;
                }
            },
            _ => { return false; },
        }
    }
    balance == 0
}

fn main() {
    let brackets = "((()))";
    println!("{}", correct_bracketing_parenthesis(brackets)); // Prints true

    let brackets = "(()";
    println!("{}", correct_bracketing_parenthesis(brackets)); // Prints false

    let brackets = "())";
    println!("{}", correct_bracketing_parenthesis(brackets)); // Prints false
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
