fn main() {}

/*

    In this Kata, you have to sort an array of non-negative integers according to
    number of ones in their binary representation in ascending order.
    For similar number of ones, sort based on decimal value.

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

fn sort_array(array: Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = array.clone();

    if array.len() == 0 {
        return res;
    }

    if (array[0] + array[array.len() - 1]) % 2 == 0 {
        res.sort();
        return res.into_iter().rev().collect();
    } else {
        res.sort();
        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_array() {
        assert!(sort_array(vec![]) == vec![]);
        assert!(sort_array(vec![5]) == vec![5]);
        assert!(sort_array(vec![2, 4, 3, 0, 1, 5]) == vec![0, 1, 2, 3, 4, 5]);
        assert!(sort_array(vec![2, 4, 3, 0, 1, 5, 6]) == vec![6, 5, 4, 3, 2, 1, 0]);
        assert!(sort_array(vec![2, 1]) == vec![1, 2]);
        assert!(sort_array(vec![15, 42, 87, 32, 11, 0]) == vec![0, 11, 15, 32, 42, 87]);
        assert!(sort_array(vec![21, 14, 23, 11]) == vec![23, 21, 14, 11]);
    }
}
