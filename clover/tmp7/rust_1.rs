
fn separate_paren_groups(paren_string: String) -> Vec<String> {
    let mut groups = Vec::new();
    let mut count = 0;
    let mut current_group = String::new();
    for c in paren_string.chars().filter(|c| !c.is_whitespace()) {
        current_group.push(c);
        if c == '(' {
            count += 1;
        } else if c == ')' {
            count -= 1;
            if count == 0 {
                groups.push(current_group);
                current_group = String::new();
            }
        }
    }
    groups
}

fn main() {
    // Example usage:
    let input = String::from("((()))(())()(()())");
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
