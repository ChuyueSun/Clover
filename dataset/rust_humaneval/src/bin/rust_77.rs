fn main() {}

/*

    Write a function that takes an integer a and returns True
    if this ingeger is a cube of some integer number.
    Note: you may assume the input is always valid.

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

fn iscuber(a: i32) -> bool {
    let a1: f64 = i32::abs(a) as f64;
    let sqrt_3 = f64::powf(a1, 1.0 / 3.0).ceil();

    return i32::pow(sqrt_3 as i32, 3) == a1 as i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iscuber() {
        assert!(iscuber(1) == true);
        assert!(iscuber(2) == false);
        assert!(iscuber(-1) == true);
        assert!(iscuber(64) == true);
        assert!(iscuber(180) == false);
        assert!(iscuber(1000) == true);
        assert!(iscuber(0) == true);
        assert!(iscuber(1729) == false);
    }
}
