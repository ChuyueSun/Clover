
fn separate_paren_groups(paren_string: String) -> Vec<String> {
    let mut result = Vec::new();
    let mut stack = 0;
    let mut current_group = String::new();

    for ch in paren_string.chars().filter(|c| !c.is_whitespace()) {
        if ch == '(' {
            stack += 1;
        }
        if stack > 0 {
            current_group.push(ch);
        }
        if ch == ')' {
            stack -= 1;
            if stack == 0 {
                result.push(current_group.clone());
                current_group.clear();
            }
        }
    }

    result
}

fn main() {
    let input = "(()) () ((()))".to_string();
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
