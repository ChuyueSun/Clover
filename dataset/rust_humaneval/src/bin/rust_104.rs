fn main() {}

/*
Given a list of positive integers x. return a sorted list of all
    elements that hasn't any even digit.

    Note: Returned list should be sorted in increasing order.

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

fn unique_digits(x: Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = vec![];
    for (_, elem) in x.into_iter().enumerate() {
        let mut elem_cp: i32 = elem;
        let mut u: bool = true;
        if elem == 0 {
            u = false;
        }
        while elem_cp > 0 && u {
            if elem_cp % 2 == 0 {
                u = false;
            }
            elem_cp = elem_cp / 10;
        }
        if u {
            res.push(elem)
        };
    }
    res.sort();
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_digits() {
        assert!(unique_digits(vec![15, 33, 1422, 1]) == vec![1, 15, 33]);
        assert!(unique_digits(vec![152, 323, 1422, 10]) == vec![]);
        assert!(unique_digits(vec![12345, 2033, 111, 151]) == vec![111, 151]);
        assert!(unique_digits(vec![135, 103, 31]) == vec![31, 135]);
    }
}
