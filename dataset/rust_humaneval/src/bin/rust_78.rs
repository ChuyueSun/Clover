fn main() {}

/*
You have been tasked to write a function that receives
    a hexadecimal number as a string and counts the number of hexadecimal
    digits that are primes (prime number, or a prime, is a natural number
    greater than 1 that is not a product of two smaller natural numbers).
    Hexadecimal digits are 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, A, B, C, D, E, F.
    Prime numbers are 2, 3, 5, 7, 11, 13, 17,...
    So you have to determine a number of the following digits: 2, 3, 5, 7,
    B (=decimal 11), D (=decimal 13).
    Note: you may assume the input is always correct or empty string,
    and symbols A,B,C,D,E,F are always uppercase.

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

fn hex_key(num: &str) -> i32 {
    let primes: Vec<&str> = vec!["2", "3", "5", "7", "B", "D"];
    let mut total: i32 = 0;
    for i in 0..num.len() {
        if primes.contains(&num.get(i..i + 1).unwrap()) {
            total += 1;
        }
    }
    return total;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_key() {
        assert!(hex_key("AB") == 1);
        assert!(hex_key("1077E") == 2);
        assert!(hex_key("ABED1A33") == 4);
        assert!(hex_key("2020") == 2);
        assert!(hex_key("123456789ABCDEF0") == 6);
        assert!(hex_key("112233445566778899AABBCCDDEEFF00") == 12);
        assert!(hex_key("") == 0);
    }
}
