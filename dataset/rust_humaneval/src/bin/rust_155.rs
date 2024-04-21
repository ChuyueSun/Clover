fn main() {}

/*
Given an integer. return a tuple that has the number of even and odd digits respectively.

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

fn even_odd_count(num: i32) -> Vec<i32> {
    let w = num.abs().to_string();
    let mut n1 = 0;
    let mut n2 = 0;
    for i in 0..w.len() {
        if w.chars().nth(i).unwrap().to_digit(10).unwrap() % 2 == 1 {
            n1 += 1;
        } else {
            n2 += 1;
        }
    }
    vec![n2, n1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_even_odd() {
        assert_eq!(even_odd_count(7), vec![0, 1]);
        assert_eq!(even_odd_count(-78), vec![1, 1]);
        assert_eq!(even_odd_count(3452), vec![2, 2]);
        assert_eq!(even_odd_count(346211), vec![3, 3]);
        assert_eq!(even_odd_count(-345821), vec![3, 3]);
        assert_eq!(even_odd_count(-2), vec![1, 0]);
        assert_eq!(even_odd_count(-45347), vec![2, 3]);
        assert_eq!(even_odd_count(0), vec![1, 0]);
    }
}
