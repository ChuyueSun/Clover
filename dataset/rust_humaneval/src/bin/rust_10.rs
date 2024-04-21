fn main() {}

/*
 Find the shortest palindrome that begins with a supplied string.
    Algorithm idea is simple:
    - Find the longest postfix of supplied string that is a palindrome.
    - Append to the end of the string reverse of a string prefix that comes before the palindromic suffix.

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

fn is_palindrome_10(str: &str) -> bool {
    let s: String = str.chars().rev().collect();
    return s == str;
}

fn make_palindrome(str: &str) -> String {
    let mut i: usize = 0;
    for i in 0..str.len() {
        let rstr: &str = &str[i..];
        if is_palindrome_10(rstr) {
            let nstr: &str = &str[0..i];
            let n2str: String = nstr.chars().rev().collect();
            return str.to_string() + &n2str;
        }
    }
    let n2str: String = str.chars().rev().collect();
    return str.to_string() + &n2str;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_palindrome() {
        assert_eq!(make_palindrome(""), "");
        assert_eq!(make_palindrome("x"), "x");
        assert_eq!(make_palindrome("xyz"), "xyzyx");
        assert_eq!(make_palindrome("xyx"), "xyx");
        assert_eq!(make_palindrome("jerry"), "jerryrrej");
    }
}
