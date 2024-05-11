
fn parse_nested_parens(paren_string: String) -> Vec<usize> {
    paren_string
        .trim()
        .split_whitespace()
        .map(|group| {
            group.chars().fold((0, 0), |(max_depth, current_depth), c| {
                match c {
                    '(' => (max(max_depth, current_depth + 1), current_depth + 1),
                    ')' => (max_depth, current_depth - 1),
                    _ => (max_depth, current_depth),
                }
            }).0
        })
        .collect()
}

fn max(a: usize, b: usize) -> usize {
    if a > b { a } else { b }
}

fn main() {
    // Example test case
    let test_string = String::from("(()()) ((())) (()) ()");
    let depths = parse_nested_parens(test_string);
    for depth in depths {
        println!("{}", depth);
    }
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
