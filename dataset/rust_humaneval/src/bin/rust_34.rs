fn main() {}

/*
Return sorted unique elements in a list

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

fn unique(nmbs: Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = nmbs.clone();
    res.sort();
    res.dedup();
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique() {
        assert!(unique(vec![5, 3, 5, 2, 3, 3, 9, 0, 123]) == vec![0, 2, 3, 5, 9, 123]);
    }
}
