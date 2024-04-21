fn main() {}

/*
 Input are two strings a and b consisting only of 1s and 0s.
    Perform binary XOR on these inputs and return result also as a string.

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

fn string_xor(a: String, b: String) -> String {
    let xor = |i: char, j: char| {
        if i == j {
            return "0".to_string();
        } else {
            return "1".to_string();
        }
    };
    return a
        .chars()
        .into_iter()
        .zip(b.chars().into_iter())
        .map(|(i, j)| "".to_string() + &xor(i, j))
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_xor() {
        assert!(string_xor("111000".to_string(), "101010".to_string()) == "010010");
        assert!(string_xor("1".to_string(), "1".to_string()) == "0");
        assert!(string_xor("0101".to_string(), "0000".to_string()) == "0101");
    }
}
