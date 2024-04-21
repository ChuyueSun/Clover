
fn main(){ 
 } 

/*
Given a string s and a natural number n, you have been tasked to implement 
    a function that returns a list of all words from string s that contain exactly 
    n consonants, in order these words appear in the string s.
    If the string s is empty then the function should return an empty list.
    Note: you may assume the input string contains only letters and spaces.
    
*/

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn select_words(s:&str, n:i32) -> Vec<String>{


    let vowels = "aeiouAEIOU";
    let mut current = String::new();
    let mut out = Vec::new();
    let mut numc = 0;
    let mut s = s.to_string();
    s.push(' ');
    for i in 0..s.len() {
        if s.chars().nth(i).unwrap() == ' ' {
            if numc == n {
                out.push(current);
            }
            current = String::new();
            numc = 0;
        } else {
            current.push(s.chars().nth(i).unwrap());
            if (s.chars().nth(i).unwrap() >= 'A' && s.chars().nth(i).unwrap() <= 'Z')
                || (s.chars().nth(i).unwrap() >= 'a' && s.chars().nth(i).unwrap() <= 'z')
            {
                if !vowels.contains(s.chars().nth(i).unwrap()) {
                    numc += 1;
                }
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_words() {
        assert_eq!(select_words("Mary had a little lamb", 4), vec!["little"]);
        assert_eq!(
            select_words("Mary had a little lamb", 3),
            vec!["Mary", "lamb"]
        );
        let v_empty: Vec<&str> = vec![];
        assert_eq!(select_words("simple white space", 2), v_empty);
        assert_eq!(select_words("Hello world", 4), vec!["world"]);
        assert_eq!(select_words("Uncle sam", 3), vec!["Uncle"]);
        assert_eq!(select_words("", 4), v_empty);
        assert_eq!(select_words("a b c d e f", 1), vec!["b", "c", "d", "f"]);
    }

}
