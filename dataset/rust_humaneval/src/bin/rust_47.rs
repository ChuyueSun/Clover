fn main() {}

/*
Return median of elements in the list l.

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

fn median(l: Vec<i32>) -> f64 {
    let mut res: Vec<i32> = l.clone();
    res.sort();
    if res.len() % 2 == 1 {
        return *res.get(res.len() / 2).unwrap() as f64;
    } else {
        return (res.get(res.len() / 2 - 1).unwrap() + res.get(res.len() / 2).unwrap()) as f64
            / 2.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_median() {
        assert!(median(vec![3, 1, 2, 4, 5]) == 3.0);
        assert!(median(vec![-10, 4, 6, 1000, 10, 20]) == 8.0);
        assert!(median(vec![5]) == 5.0);
        assert!(median(vec![6, 5]) == 5.5);
        assert!(median(vec![8, 1, 3, 9, 9, 2, 7]) == 7.0);
    }
}
