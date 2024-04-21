fn main() {}

/*
 Given a string, find out how many distinct characters (regardless of case) does it consist of

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

fn count_distinct_characters(str: String) -> i32 {
    let res: HashSet<char> = str
        .chars()
        .into_iter()
        .map(|x: char| x.to_ascii_lowercase())
        .collect();
    return res.len() as i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_distinct_characters() {
        assert!(count_distinct_characters("".to_string()) == 0);
        assert!(count_distinct_characters("abcde".to_string()) == 5);
        assert!(
            count_distinct_characters(
                "abcde".to_string() + &"cade".to_string() + &"CADE".to_string()
            ) == 5
        );
        assert!(count_distinct_characters("aaaaAAAAaaaa".to_string()) == 1);
        assert!(count_distinct_characters("Jerry jERRY JeRRRY".to_string()) == 5);
    }
}
