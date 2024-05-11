
fn match_parens(lst: Vec<&str>) -> String {
    let (mut first_open, mut first_close) = (0, 0);
    let (mut second_open, mut second_close) = (0, 0);

    // Check if the inputs are valid
    if lst.len() != 2 {
        return "No".to_string();
    }

    for c in lst[0].chars() {
        match c {
            '(' => first_open += 1,
            ')' => first_close += 1,
            _ => {}
        }
    }

    for c in lst[1].chars() {
        match c {
            '(' => second_open += 1,
            ')' => second_close += 1,
            _ => {}
        }
    }

    // Check if the second can be appended after the first
    let can_append_second_after_first = if first_open >= first_close {
        (first_open - first_close) >= second_close
    } else {
        false
    };

    // Check if the first can be appended after the second
    let can_append_first_after_second = if second_open >= second_close {
        (second_open - second_close) >= first_close
    } else {
        false
    };

    if can_append_second_after_first || can_append_first_after_second {
        "Yes".to_string()
    } else {
        "No".to_string()
    }
}

// Example usage
fn main() {
    let lst1 = vec!["(()", "))()"];
    println!("{}", match_parens(lst1)); // Should print "Yes"

    let lst2 = vec![")(", ")("];
    println!("{}", match_parens(lst2)); // Should print "No" 
}
`
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
