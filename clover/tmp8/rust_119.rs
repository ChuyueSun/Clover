
fn match_parens(lst: Vec<&str>) -> String {
    if lst.len() != 2 {
        return "No".to_string();
    }

    let balance = |s: &str| s.chars().fold(0, |acc, c| if c == '(' { acc + 1 } else { acc - 1 });

    let first_balance = balance(lst[0]);
    let second_balance = balance(lst[1]);

    if first_balance == 0 && second_balance == 0 {
        return "Yes".to_string();
    }

    if first_balance > 0 && second_balance > 0 || first_balance < 0 && second_balance < 0 {
        return "No".to_string();
    }

    if first_balance > 0 {
        if lst[0].chars().last().unwrap() == '(' || lst[1].chars().next().unwrap() == ')' {
            "Yes".to_string()
        } else {
            "No".to_string()
        }
    } else if lst[0].chars().next().unwrap() == ')' || lst[1].chars().last().unwrap() == '(' {
        "Yes".to_string()
    } else {
        "No".to_string()
    }
}

fn main() {
    // Example usage
    let result = match_parens(vec!["(()", ")()"]);
    println!("{}", result); // Outputs: Yes
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
