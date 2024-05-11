
fn match_parens(lst: Vec<&str>) -> String {
    if lst.len() != 2 {
        return "No".to_string();
    }

    let s1 = lst[0];
    let s2 = lst[1];

    // Count the number of open and close parentheses for each string
    let count_parens = |s: &str| -> (i32, i32) {
        s.chars().fold((0, 0), |(open, close), c| {
            match c {
                '(' => (open + 1, close),
                ')' => (open, close + 1),
                _ => (open, close),
            }
        })
    };
    
    let (open1, close1) = count_parens(s1);
    let (open2, close2) = count_parens(s2);

    // To form a good string by concatenating, the number of open
    // parentheses in the first string must be greater or equal to the 
    // number of close parentheses in the second string
    if open1 >= close2 && open1 + open2 == close1 + close2 {
        "Yes".to_string()
    } else if open2 >= close1 && open1 + open2 == close1 + close2 {
        "Yes".to_string()
    } else {
        "No".to_string()
    }
}

// You can call the function with an example like this:
// let result = match_parens(vec!["(", ")"]); 
// println!("{}", result); // Output would be "Yes"

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
