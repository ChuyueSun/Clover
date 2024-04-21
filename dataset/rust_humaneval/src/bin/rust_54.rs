fn main() {}

/*

    Check if two words have the same characters.

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

fn same_chars(str1: &str, str2: &str) -> bool {
    let mut v1: Vec<char> = str1.chars().into_iter().collect();
    v1.sort();
    v1.dedup();

    let mut v2: Vec<char> = str2.chars().into_iter().collect();
    v2.sort();
    v2.dedup();

    return v1 == v2;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_same_chars() {
        assert!(same_chars("eabcdzzzz", "dddzzzzzzzddeddabc") == true);
        assert!(same_chars("abcd", "dddddddabc") == true);
        assert!(same_chars("dddddddabc", "abcd") == true);
        assert!(same_chars("eabcd", "dddddddabc") == false);
        assert!(same_chars("abcd", "dddddddabcf") == false);
        assert!(same_chars("eabcdzzzz", "dddzzzzzzzddddabc") == false);
        assert!(same_chars("aabb", "aaccc") == false);
    }
}
