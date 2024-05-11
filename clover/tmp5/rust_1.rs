
fn separate_paren_groups(paren_string: String) -> Vec<String> {
    let mut groups = Vec::new();
    let mut group_start = 0;
    let mut depth = 0;

    for (i, char) in paren_string.chars().enumerate() {
        match char {
            '(' => {
                if depth == 0 {
                    group_start = i;
                }
                depth += 1;
            }
            ')' => {
                depth -= 1;
                if depth == 0 {
                    groups.push(paren_string[group_start..=i].to_string());
                }
            }
            ' ' => {} // Ignore spaces
            _ => panic!("Invalid character in input string"),
        }
    }

    if depth != 0 {
        panic!("Unbalanced parentheses in input string");
    }

    groups
}

fn main() {
    let input = "(()()) (()) ()".to_string();
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
