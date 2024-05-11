
fn match_parens(lst: Vec<&str>) -> String {
    // Since there are only two elements, we can directly access them and store the balance of parentheses.
    let mut balance_first = 0;
    let mut balance_second = 0;
    
    // Count balance for the first string
    for ch in lst[0].chars() {
        match ch {
            '(' => balance_first += 1,
            ')' => balance_first -= 1,
            _ => {}
        }
        // If the balance goes negative, the string can't be made good
        if balance_first < 0 {
            break;
        }
    }
    
    // Count balance for the second string
    for ch in lst[1].chars() {
        match ch {
            '(' => balance_second += 1,
            ')' => balance_second -= 1,
            _ => {}
        }
        // If the balance goes negative, the string can't be made good
        if balance_second < 0 {
            break;
        }
    }
    
    // If both strings are individually balanced or if the sum of balances is 0 and the first string doesn't end with a negative balance
    // it means we can concatenate them in some order to make a good string.
    if (balance_first == 0 && balance_second == 0) || 
       (balance_first >= 0 && balance_second + balance_first == 0) {
        "Yes".to_string()
    } else {
        "No".to_string()
    }
}

// Example usage:
// let result = match_parens(vec!["(", ")"]);
// assert_eq!(result, "Yes");

// let result = match_parens(vec!["(()", "())"]);
// assert_eq!(result, "Yes");

// let result = match_parens(vec!["(", ")("]);
// assert_eq!(result, "No");

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
