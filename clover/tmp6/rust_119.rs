
fn match_parens(lst: Vec<&str>) -> &'static str {
    let (a, b) = (lst[0], lst[1]);
    let balance_a = a.chars().fold(0, |acc, c| if c == '(' { acc + 1 } else { acc - 1 });
    let balance_b = b.chars().fold(0, |acc, c| if c == '(' { acc + 1 } else { acc - 1 });
    
    if balance_a == 0 && balance_b == 0 {
        return "Yes";
    }
    
    let a_starts_with_open = a.starts_with('(');
    let b_ends_with_close = b.ends_with(')');
    
    if (balance_a >= 0 && a_starts_with_open && balance_b <= 0 && b_ends_with_close) || 
       (balance_b >= 0 && b.starts_with('(') && balance_a <= 0 && a.ends_with(')')) {
        return "Yes";
    }

    "No"
}

fn main() {
    println!("{}", match_parens(vec!["(", ")"])); // Yes
    println!("{}", match_parens(vec!["(()", ")()"])); // Yes
    println!("{}", match_parens(vec!["(", ")))"])); // No
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
