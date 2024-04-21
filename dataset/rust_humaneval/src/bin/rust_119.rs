fn main() {}

/*

    You are given a list of two strings, both strings consist of open
    parentheses '(' or close parentheses ')' only.
    Your job is to check if it is possible to concatenate the two strings in
    some order, that the resulting string will be good.
    A string S is considered to be good if and only if all parentheses in S
    are balanced. For example: the string '(())()' is good, while the string
    '())' is not.
    Return 'Yes' if there's a way to make a good string, and return 'No' otherwise.

*/

use md5;
use rand::Rng;
use regex::Regex;
use std::any::{Any, TypeId};
use std::{
    ascii::AsciiExt,
    cmp::{self, max},
    collections::{HashMap, HashSet},
    mem::replace,
    ops::Index,
    slice::Iter,
};

fn match_parens(lst: Vec<&str>) -> &str {
    let l1 = lst[0].to_string() + lst[1];
    let mut count = 0;
    let mut can = true;
    for i in 0..l1.len() {
        if l1.chars().nth(i).unwrap() == '(' {
            count += 1;
        }
        if l1.chars().nth(i).unwrap() == ')' {
            count -= 1;
        }
        if count < 0 {
            can = false;
        }
    }
    if count != 0 {
        return "No";
    }
    if can == true {
        return "Yes";
    }
    let l1 = lst[1].to_string() + lst[0];
    let mut can = true;
    for i in 0..l1.len() {
        if l1.chars().nth(i).unwrap() == '(' {
            count += 1;
        }
        if l1.chars().nth(i).unwrap() == ')' {
            count -= 1;
        }
        if count < 0 {
            can = false;
        }
    }
    if can == true {
        return "Yes";
    }
    return "No";
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
