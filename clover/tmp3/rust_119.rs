
fn match_parens(lst: Vec<&str>) -> String {
    if lst.len() != 2 {
        return "No".to_string();
    }

    let check_good = |s: &str| -> bool {
        let mut balance = 0;
        for c in s.chars() {
            if c == '(' {
                balance += 1;
            } else if c == ')' {
                if balance == 0 {
                    return false;
                }
                balance -= 1;
            }
        }
        balance == 0
    };

    if check_good(&(lst[0].to_string() + lst[1])) || check_good(&(lst[1].to_string() + lst[0])) {
        "Yes".to_string()
    } else {
        "No".to_string()
    }
}

fn main() {
    // Example usage:
    // The strings in the list can be concatenated in the order `[1, 0]` to form the string "()()()" which is good.
    println!("{}", match_parens(vec!["()", "()()"])); // "Yes"
    // There is no way to concatenate the strings to form a good string.
    println!("{}", match_parens(vec!["())", "(()"])); // "No"
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
