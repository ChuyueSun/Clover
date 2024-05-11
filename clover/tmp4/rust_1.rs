
fn separate_paren_groups(paren_string: String) -> Vec<String> {
    let mut groups = Vec::new();
    let mut balance = 0;
    let mut group_start = None;

    for (i, ch) in paren_string.chars().enumerate() {
        match ch {
            '(' => {
                if balance == 0 {
                    group_start = Some(i);
                }
                balance += 1;
            }
            ')' => {
                balance -= 1;
                if balance == 0 {
                    if let Some(start) = group_start {
                        groups.push(paren_string[start..=i].to_string());
                        group_start = None;
                    }
                }
            }
            ' ' => {} // ignore spaces
            _ => panic!("Invalid character in input string"), // or handle the error as appropriate
        }
    }
    
    if balance != 0 {
        panic!("Unbalanced parentheses in input string"); // or handle the error as appropriate
    }

    groups
}

fn main() {
    let input = "((())(())) (()) ()".to_string();
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
