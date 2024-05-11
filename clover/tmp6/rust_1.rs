
fn separate_paren_groups(paren_string: String) -> Vec<String> {
    let mut result = Vec::new();
    let mut depth = 0;
    let mut current_group = String::new();

    for c in paren_string.chars() {
        if c == ' ' {
            continue;
        }

        if c == '(' {
            depth += 1;
        } else if c == ')' {
            depth -= 1;
        }

        current_group.push(c);

        if depth == 0 && !current_group.is_empty() {
            result.push(current_group.clone());
            current_group.clear();
        }
    }

    result
}

fn main() {
    // Example usage:
    let input = "((()))(())()".to_string();
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
