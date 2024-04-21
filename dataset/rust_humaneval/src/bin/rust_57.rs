fn main() {}

/*
Return True is list elements are monotonically increasing or decreasing.

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

fn monotonic(l: Vec<i32>) -> bool {
    let mut l1: Vec<i32> = l.clone();
    let mut l2: Vec<i32> = l.clone();
    l2.sort();
    l2.reverse();
    l1.sort();

    if l == l1 || l == l2 {
        return true;
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monotonic() {
        assert!(monotonic(vec![1, 2, 4, 10]) == true);
        assert!(monotonic(vec![1, 2, 4, 20]) == true);
        assert!(monotonic(vec![1, 20, 4, 10]) == false);
        assert!(monotonic(vec![4, 1, 0, -10]) == true);
        assert!(monotonic(vec![4, 1, 1, 0]) == true);
        assert!(monotonic(vec![1, 2, 3, 2, 5, 60]) == false);
        assert!(monotonic(vec![1, 2, 3, 4, 5, 60]) == true);
        assert!(monotonic(vec![9, 9, 9, 9]) == true);
    }
}
