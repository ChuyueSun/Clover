fn main() {}

/*
Given a positive integer n, return the product of the odd digits.
    Return 0 if all digits are even.

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

fn digits(n: i32) -> i32 {
    let mut prod: i32 = 1;
    let mut has = 0;
    let s = n.to_string();
    for i in 0..s.len() {
        if s.chars().nth(i).unwrap().to_digit(10).unwrap() % 2 == 1 {
            has = 1;
            prod = prod * (s.chars().nth(i).unwrap().to_digit(10).unwrap()) as i32;
        }
    }
    if has == 0 {
        return 0;
    }
    prod
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digits() {
        assert_eq!(digits(5), 5);
        assert_eq!(digits(54), 5);
        assert_eq!(digits(120), 1);
        assert_eq!(digits(5014), 5);
        assert_eq!(digits(98765), 315);
        assert_eq!(digits(5576543), 2625);
        assert_eq!(digits(2468), 0);
    }
}
