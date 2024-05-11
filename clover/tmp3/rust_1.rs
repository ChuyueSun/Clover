
fn separate_paren_groups(paren_string: String) -> Vec<String> {
    let mut groups = Vec::new();
    let mut group_stack = Vec::new();
    let mut current_group = String::new();

    for c in paren_string.chars().filter(|c| !c.is_whitespace()) {
        current_group.push(c);
        match c {
            '(' => group_stack.push(c),
            ')' => {
                if let Some(_) = group_stack.pop() {
                    if group_stack.is_empty() {
                        groups.push(current_group.clone());
                        current_group.clear();
                    }
                } else {
                    panic!("Unbalanced parentheses detected.");
                }
            }
            _ => panic!("Invalid character, expected only parentheses."),
        }
    }

    if !group_stack.is_empty() {
        panic!("Unbalanced parentheses in the input string.");
    }

    groups
}

fn main() {
    // Example usage:
    let input = "(()) () ((()))".to_string();
    let output = separate_paren_groups(input);
    println!("Separated groups: {:?}", output);
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
