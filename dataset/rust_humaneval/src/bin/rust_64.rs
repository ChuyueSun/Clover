fn main() {}

/*
Write a function vowels_count which takes a string representing
    a word as input and returns the number of vowels in the string.
    Vowels in this case are 'a', 'e', 'i', 'o', 'u'. Here, 'y' is also a
    vowel, but only when it is at the end of the given word.

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

fn vowels_count(s: &str) -> i32 {
    let vowels: &str = "aeiouAEIOU";
    let mut count: i32 = 0;

    for i in 0..s.len() {
        let c: char = s.chars().nth(i).unwrap();
        if vowels.contains(c) {
            count += 1;
        }
    }
    if s.chars().nth(s.len() - 1).unwrap() == 'y' || s.chars().nth(s.len() - 1).unwrap() == 'Y' {
        count += 1;
    }

    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vowels_count() {
        assert!(vowels_count("abcde") == 2);
        assert!(vowels_count("Alone") == 3);
        assert!(vowels_count("key") == 2);
        assert!(vowels_count("bye") == 1);
        assert!(vowels_count("keY") == 2);
        assert!(vowels_count("bYe") == 1);
        assert!(vowels_count("ACEDY") == 3);
    }
}
