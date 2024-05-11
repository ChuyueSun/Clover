
fn separate_paren_groups(paren_string: String) -> Vec<String> {
    let mut groups: Vec<String> = Vec::new();
    let mut group_stack: Vec<String> = Vec::new();
    let mut current_group = String::new();
    let mut depth = 0;

    for c in paren_string.chars().filter(|c| !c.is_whitespace()) {
        match c {
            '(' => {
                depth += 1;
                current_group.push(c);
            },
            ')' => {
                if depth == 0 {
                    panic!("Unmatched closing parenthesis detected!");
                }
                current_group.push(c);
                depth -= 1;
                if depth == 0 {
                    group_stack.push(current_group.clone());
                    current_group.clear();
                }
            },
            _ => panic!("Invalid character detected, only parentheses are allowed!"),
        }
    }

    if depth != 0 {
        panic!("Unmatched opening parenthesis detected!");
    }

    while let Some(group) = group_stack.pop() {
        groups.push(group);
    }

    groups.reverse();
    groups
}

fn main() {
    let example = "(()) (()()) (())".to_string();
    let groups = separate_paren_groups(example);
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
