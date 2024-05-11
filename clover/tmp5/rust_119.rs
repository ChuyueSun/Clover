
fn match_parens(lst: Vec<&str>) -> String {
    let mut balance = 0;
    let mut max_balance = 0;

    for &s in &lst {
        for c in s.chars() {
            balance += if c == '(' { 1 } else { -1 };
            if balance < 0 {
                return "No".to_string();
            }
            max_balance = max_balance.max(balance);
        }
    }

    if balance != 0 {
        return "No".to_string();
    }

    balance = 0;
    for &s in lst.iter().rev() {
        for c in s.chars() {
            balance += if c == ')' { 1 } else { -1 };
            if balance < 0 {
                return "No".to_string();
            }
        }
    }

    "Yes".to_string()
}

fn main() {
    assert_eq!(match_parens(vec!["(", ")"]), "Yes".to_string());
    assert_eq!(match_parens(vec!["(", "("]), "No".to_string());
    assert_eq!(match_parens(vec![")(", ")("]), "No".to_string());
    assert_eq!(match_parens(vec!["()(", ")"]), "Yes".to_string());
    assert_eq!(match_parens(vec!["(()", ")"]), "Yes".to_string());
    println!("All tests passed!");
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
