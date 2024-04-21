fn main() {}

/*
 For a given number n, find the largest number that divides n evenly, smaller than n

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

fn largest_divisor(n: i32) -> i32 {
    let mut res: i32 = 0;
    let sqn = 1..n;

    for i in sqn.rev() {
        if n % i == 0 {
            res = i;
            break;
        }
    }

    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_divisor() {
        assert!(largest_divisor(3) == 1);
        assert!(largest_divisor(7) == 1);
        assert!(largest_divisor(10) == 5);
        assert!(largest_divisor(100) == 50);
        assert!(largest_divisor(49) == 7);
    }
}
