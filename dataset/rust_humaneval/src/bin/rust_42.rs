fn main() {}

/*
Return list with elements incremented by 1.

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

fn incr_list(l: Vec<i32>) -> Vec<i32> {
    return l.into_iter().map(|n: i32| n + 1).collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_incr_list() {
        assert!(incr_list(vec![]) == vec![]);
        assert!(incr_list(vec![3, 2, 1]) == [4, 3, 2]);
        assert!(incr_list(vec![5, 2, 5, 2, 3, 3, 9, 0, 123]) == [6, 3, 6, 3, 4, 4, 10, 1, 124]);
    }
}
