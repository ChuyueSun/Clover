fn main() {}

/*

    You are given a string representing a sentence,
    the sentence contains some words separated by a space,
    and you have to return a string that contains the words from the original sentence,
    whose lengths are prime numbers,
    the order of the words in the new string should be the same as the original one.

    Constraints:
        * 1 <= len(sentence) <= 100
        * sentence contains only letters

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

fn words_in_sentence(sentence: &str) -> String {
    let mut out = String::new();
    let mut current = String::new();
    let mut sentence = sentence.to_string();
    sentence.push(' ');

    for i in 0..sentence.len() {
        if sentence.chars().nth(i).unwrap() != ' ' {
            current.push(sentence.chars().nth(i).unwrap());
        } else {
            let mut isp = true;
            let l = current.len();
            if l < 2 {
                isp = false;
            }
            for j in 2..(l as f64).sqrt() as usize + 1 {
                if l % j == 0 {
                    isp = false;
                }
            }
            if isp {
                out.push_str(&current);
                out.push(' ');
            }
            current = String::new();
        }
    }
    if out.len() > 0 {
        out.pop();
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_words_in_sentence() {
        assert_eq!(words_in_sentence("This is a test"), "is");
        assert_eq!(words_in_sentence("lets go for swimming"), "go for");
        assert_eq!(
            words_in_sentence("there is no place available here"),
            "there is no place"
        );
        assert_eq!(words_in_sentence("Hi I am Hussein"), "Hi am Hussein");
        assert_eq!(words_in_sentence("go for it"), "go for it");
        assert_eq!(words_in_sentence("here"), "");
        assert_eq!(words_in_sentence("here is"), "is");
    }
}
