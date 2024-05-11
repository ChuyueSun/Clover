
fn separate_paren_groups(paren_string: String) -> Vec<String> {
    let mut groups = Vec::new();
    let mut depth = 0;
    let mut current_group = String::new();
    
    for c in paren_string.chars().filter(|c| !c.is_whitespace()) {
        if c == '(' {
            depth += 1;
            current_group.push(c);
        } else if c == ')' {
            depth -= 1;
            current_group.push(c);
            if depth == 0 {
                groups.push(current_group.clone());
                current_group.clear();
            }
        }
    }
    groups
}

fn main() {
    // Test the function
    let input = "(()) () ((())) (())".to_string();
    let separated_groups = separate_paren_groups(input);
    for group in separated_groups {
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
