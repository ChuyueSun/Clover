
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
            _ => {}
        }
    }
    balance == 0
}

fn main() {
    // You can test your function here
    println!("{}", correct_bracketing_parenthesis("(()())")); // should print true
    println!("{}", correct_bracketing_parenthesis("(()")); // should print false
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
