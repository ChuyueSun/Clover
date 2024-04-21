
fn main(){ 
 } 

/*

    Checks if given string is a palindrome
    
*/

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn is_palindrome(text: String) -> bool {


    let pr: String = text.chars().rev().collect();
    return pr == text;
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome("".to_string()) == true);
        assert!(is_palindrome("aba".to_string()) == true);
        assert!(is_palindrome("aaaaa".to_string()) == true);
        assert!(is_palindrome("zbcd".to_string()) == false);
        assert!(is_palindrome("xywyx".to_string()) == true);
        assert!(is_palindrome("xywyz".to_string()) == false);
        assert!(is_palindrome("xywzx".to_string()) == false);
    }

}
