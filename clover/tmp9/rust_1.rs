
fn separate_paren_groups(paren_string: String) -> Vec<String> {
    let mut groups = Vec::new();
    let mut start = 0;
    let mut open_braces = 0;
    let chars: Vec<char> = paren_string.chars().filter(|c| *c != ' ').collect();

    for (i, &ch) in chars.iter().enumerate() {
        match ch {
            '(' => open_braces += 1,
            ')' => {
                open_braces -= 1;
                if open_braces == 0 {
                    groups.push(chars[start..=i].iter().collect());
                    start = i + 1;
                }
            }
            _ => {}
        }
    }

    groups
}

fn main() {
    // Example usage:
    let input = "((())()) (()(())) (())".to_string();
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
