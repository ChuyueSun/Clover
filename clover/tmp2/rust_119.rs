
fn match_parens(lst: Vec<&str>) -> &'static str {
    let (left_counts, right_counts): (Vec<_>, Vec<_>) = lst
        .iter()
        .map(|s| s.chars().fold((0, 0), |(l, r), c| if c == '(' { (l + 1, r) } else { (l, r + 1) }))
        .unzip();

    if (left_counts[0] >= right_counts[1] && left_counts[1] + right_counts[0] == left_counts[0] + right_counts[1])
       || (left_counts[1] >= right_counts[0] && left_counts[0] + right_counts[1] == left_counts[1] + right_counts[0]) {
        "Yes"
    } else {
        "No"
    }
}

fn main() {
    // Example usage:
    println!("{}", match_parens(vec!["(", ")"])); // Yes
    println!("{}", match_parens(vec!["(", "("])); // No
    println!("{}", match_parens(vec!["(()", ")()"])); // Yes
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
