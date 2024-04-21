fn main() {}

/*
Create a function which takes a string representing a file's name, and returns
    'Yes' if the the file's name is valid, and returns 'No' otherwise.
    A file's name is considered to be valid if and only if all the following conditions
    are met:
    - There should not be more than three digits ('0'-'9') in the file's name.
    - The file's name contains exactly one dot '.'
    - The substring before the dot should not be empty, and it starts with a letter from
    the latin alphapet ('a'-'z' and 'A'-'Z').
    - The substring after the dot should be one of these: ['txt', 'exe', 'dll']

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

fn file_name_check(file_name: &str) -> &str {
    let mut numdigit = 0;
    let mut numdot = 0;
    if file_name.len() < 5 {
        return "No";
    }
    let w = file_name.chars().nth(0).unwrap();
    if w < 'A' || (w > 'Z' && w < 'a') || w > 'z' {
        return "No";
    }
    let last = &file_name[file_name.len() - 4..];
    if last != ".txt" && last != ".exe" && last != ".dll" {
        return "No";
    }
    for c in file_name.chars() {
        if c >= '0' && c <= '9' {
            numdigit += 1;
        }
        if c == '.' {
            numdot += 1;
        }
    }
    if numdigit > 3 || numdot != 1 {
        return "No";
    }
    return "Yes";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_name_check() {
        assert_eq!(file_name_check("example.txt"), "Yes");
        assert_eq!(file_name_check("1example.dll"), "No");
        assert_eq!(file_name_check("s1sdf3.asd"), "No");
        assert_eq!(file_name_check("K.dll"), "Yes");
        assert_eq!(file_name_check("MY16FILE3.exe"), "Yes");
        assert_eq!(file_name_check("His12FILE94.exe"), "No");
        assert_eq!(file_name_check("_Y.txt"), "No");
        assert_eq!(file_name_check("?aREYA.exe"), "No");
        assert_eq!(file_name_check("/this_is_valid.dll"), "No");
        assert_eq!(file_name_check("this_is_valid.wow"), "No");
        assert_eq!(file_name_check("this_is_valid.txt"), "Yes");
        assert_eq!(file_name_check("this_is_valid.txtexe"), "No");
        assert_eq!(file_name_check("#this2_i4s_5valid.ten"), "No");
        assert_eq!(file_name_check("@this1_is6_valid.exe"), "No");
        assert_eq!(file_name_check("this_is_12valid.6exe4.txt"), "No");
        assert_eq!(file_name_check("all.exe.txt"), "No");
        assert_eq!(file_name_check("I563_No.exe"), "Yes");
        assert_eq!(file_name_check("Is3youfault.txt"), "Yes");
        assert_eq!(file_name_check("no_one#knows.dll"), "Yes");
        assert_eq!(file_name_check("1I563_Yes3.exe"), "No");
        assert_eq!(file_name_check("I563_Yes3.txtt"), "No");
        assert_eq!(file_name_check("final..txt"), "No");
        assert_eq!(file_name_check("final132"), "No");
        assert_eq!(file_name_check("_f4indsartal132."), "No");
        assert_eq!(file_name_check(".txt"), "No");
        assert_eq!(file_name_check("s."), "No");
    }
}
