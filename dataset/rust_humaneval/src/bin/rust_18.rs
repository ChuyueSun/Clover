fn main() {}

/*
 Find how many times a given substring can be found in the original string. Count overlaping cases.

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

fn how_many_times(string: String, substring: String) -> i32 {
    let mut times: i32 = 0;

    for i in 0..(string.len() as i32 - substring.len() as i32 + 1) {
        if string
            .get(i as usize..(i + substring.len() as i32) as usize)
            .unwrap()
            .to_string()
            == substring
        {
            times += 1;
        }
    }
    return times;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_how_many_times() {
        assert!(how_many_times("".to_string(), "x".to_string()) == 0);
        assert!(how_many_times("xyxyxyx".to_string(), "x".to_string()) == 4);
        assert!(how_many_times("cacacacac".to_string(), "cac".to_string()) == 4);
        assert!(how_many_times("john doe".to_string(), "john".to_string()) == 1);
    }
}
