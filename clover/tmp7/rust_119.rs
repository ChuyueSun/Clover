
fn match_parens(lst: Vec<&str>) -> String {
    if lst.len() != 2 {
        return "No".to_string();
    }

    let (mut open1, mut close1) = (0, 0);
    let (mut open2, mut close2) = (0, 0);

    for c in lst[0].chars() {
        match c {
            '(' => open1 += 1,
            ')' => close1 += 1,
            _ => (),
        }
    }

    for c in lst[1].chars() {
        match c {
            '(' => open2 += 1,
            ')' => close2 += 1,
            _ => (),
        }
    }

    if (open1 >= close2 && open1 + open2 == close1 + close2) || (open2 >= close1 && open1 + open2 == close1 + close2) {
        "Yes".to_string()
    } else {
        "No".to_string()
    }
}

fn main() {
    // You can test the function here
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
