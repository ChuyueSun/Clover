
fn main(){ 
 } 

/*

    You will be given a string of words separated by commas or spaces. Your task is
    to split the string into words and return an array of the words.
    
*/

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn words_string(s:&str) -> Vec<String> {


    return s
        .to_string()
        .split(|c: char| c == ',' || c.is_whitespace())
        .into_iter()
        .filter(|x| x != &"")
        .map(|x| x.to_string())
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_words_string() {
        assert!(words_string("Hi, my name is John") == vec!["Hi", "my", "name", "is", "John"]);
        assert!(
            words_string("One, two, three, four, five, six")
                == vec!["One", "two", "three", "four", "five", "six"]
        );
        assert!(words_string("Hi, my name") == vec!["Hi", "my", "name"]);
        assert!(
            words_string("One,, two, three, four, five, six,")
                == vec!["One", "two", "three", "four", "five", "six"]
        );
        let v_empty: Vec<String> = vec![];
        assert!(words_string("") == v_empty);
        assert!(words_string("ahmed , gamal") == vec!["ahmed", "gamal"]);
    }

}
