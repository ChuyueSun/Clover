
fn parse_nested_parens(paren_string: String) -> Vec<i32> {
    paren_string
        .split_whitespace()
        .map(|group| {
            let mut max_depth = 0;
            let mut current_depth = 0;
            for c in group.chars() {
                match c {
                    '(' => {
                        current_depth += 1;
                        if current_depth > max_depth {
                            max_depth = current_depth;
                        }
                    }
                    ')' => current_depth -= 1,
                    _ => {}
                };
            }
            max_depth
        })
        .collect()
}

fn main() {
    // Example usage of the function:
    let input = String::from("(()) (()(())) ((())) (()())");
    let max_depths = parse_nested_parens(input);
    println!("{:?}", max_depths); // Should print `[2, 3, 3, 2]`
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn test_parse_nested_parens() {
        assert!(
            parse_nested_parens(String::from("(()()) ((())) () ((())()())")) == vec![2, 3, 1, 3]
        );
        assert!(parse_nested_parens(String::from("() (()) ((())) (((())))")) == vec![1, 2, 3, 4]);
        assert!(parse_nested_parens(String::from("(()(())((())))")) == vec![4]);
    }

}
