
fn match_parens(lst: Vec<&str>) -> String {
    if lst.len() != 2 {
        return "No".to_string();
    }

    let (left_count_1, right_count_1) = count_parentheses(lst[0]);
    let (left_count_2, right_count_2) = count_parentheses(lst[1]);

    if left_count_1 >= right_count_1 && left_count_2 <= right_count_2 {
        "Yes".to_string()
    } else if left_count_2 >= right_count_2 && left_count_1 <= right_count_1 {
        "Yes".to_string()
    } else {
        "No".to_string()
    }
}

fn count_parentheses(s: &str) -> (usize, usize) {
    s.chars().fold((0, 0), |(left, right), c| match c {
        '(' => (left + 1, right),
        ')' => (left, right + 1),
        _ => (left, right),
    })
}

fn main() {
    // Example usage:
    let lst = vec!["(()", "())"];
    println!("{}", match_parens(lst));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_parens() {
        assert_eq!(match_parens(vec!["()(", ")"]), "Yes");
        assert_eq!(match_parens(vec![")", ")"]), "No");
        assert_eq!(match_parens(vec!["(()(())", "())())"],), "No");
        assert_eq!(match_parens(vec![")())", "(()()("]), "Yes");
        assert_eq!(match_parens(vec!["(())))", "(()())(("]), "Yes");
        assert_eq!(match_parens(vec!["()", "())"],), "No");
        assert_eq!(match_parens(vec!["(()(", "()))()"]), "Yes");
        assert_eq!(match_parens(vec!["((((", "((())"],), "No");
        assert_eq!(match_parens(vec![")(()", "(()("]), "No");
        assert_eq!(match_parens(vec![")(", ")("]), "No");
        assert_eq!(match_parens(vec!["(", ")"]), "Yes");
        assert_eq!(match_parens(vec![")", "("]), "Yes");
    }

}
