
fn separate_paren_groups(paren_string: String) -> Vec<String> {
    let mut groups = Vec::new();
    let mut balance = 0;
    let mut current_group = String::new();

    for ch in paren_string.chars().filter(|c| !c.is_whitespace()) {
        if ch == '(' {
            balance += 1;
        }
        if balance > 0 {
            current_group.push(ch);
        }
        if ch == ')' {
            balance -= 1;
            if balance == 0 {
                groups.push(current_group.clone());
                current_group.clear();
            }
        }
    }

    groups
}

fn main() {
    let input = "(())   () (())".to_string();
    let groups = separate_paren_groups(input);
    for group in groups {
        println!("{}", group);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn test_separate_paren_groups() {
        assert_eq!(
            separate_paren_groups(String::from("(()()) ((())) () ((())()())")),
            vec!["(()())", "((()))", "()", "((())()())"]
        );
        assert_eq!(
            separate_paren_groups(String::from("() (()) ((())) (((())))")),
            vec!["()", "(())", "((()))", "(((())))"]
        );
        assert_eq!(
            separate_paren_groups(String::from("(()(())((())))")),
            vec!["(()(())((())))"]
        );
        assert_eq!(
            separate_paren_groups(String::from("( ) (( )) (( )( ))")),
            vec!["()", "(())", "(()())"]
        );
    }

}
