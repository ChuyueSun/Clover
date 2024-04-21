
fn main(){ 
 } 

/*

    Given a positive integer, obtain its roman numeral equivalent as a string,
    and return it in lowercase.
    Restrictions: 1 <= num <= 1000
    
*/

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn int_to_mini_romank(number: i32) -> String {


    let mut current = String::new();
    let mut number = number;
    let rep = vec![
        "m", "cm", "d", "cd", "c", "xc", "l", "xl", "x", "ix", "v", "iv", "i",
    ];
    let num = vec![1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
    let mut pos = 0;
    while number > 0 {
        while number >= num[pos] {
            current.push_str(rep[pos]);
            number -= num[pos];
        }
        if number > 0 {
            pos += 1;
        }
    }
    current
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_to_mini_romank() {
        assert_eq!(int_to_mini_romank(19), "xix");
        assert_eq!(int_to_mini_romank(152), "clii");
        assert_eq!(int_to_mini_romank(251), "ccli");
        assert_eq!(int_to_mini_romank(426), "cdxxvi");
        assert_eq!(int_to_mini_romank(500), "d");
        assert_eq!(int_to_mini_romank(1), "i");
        assert_eq!(int_to_mini_romank(4), "iv");
        assert_eq!(int_to_mini_romank(43), "xliii");
        assert_eq!(int_to_mini_romank(90), "xc");
        assert_eq!(int_to_mini_romank(94), "xciv");
        assert_eq!(int_to_mini_romank(532), "dxxxii");
        assert_eq!(int_to_mini_romank(900), "cm");
        assert_eq!(int_to_mini_romank(994), "cmxciv");
        assert_eq!(int_to_mini_romank(1000), "m");
    }

}
