fn main() {}

/*

    triples_sum_to_zero takes a list of integers as an input.
    it returns True if there are three distinct elements in the list that
    sum to zero, and False otherwise.

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

fn triples_sum_to_zero(nmbs: Vec<i32>) -> bool {
    for i in 0..nmbs.len() {
        for j in i + 1..nmbs.len() {
            for k in j + 1..nmbs.len() {
                if *nmbs.get(i).unwrap() + *nmbs.get(j).unwrap() + *nmbs.get(k).unwrap() == 0 {
                    return true;
                }
            }
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triples_sum_to_zero() {
        assert!(triples_sum_to_zero(vec![1, 3, 5, 0]) == false);
        assert!(triples_sum_to_zero(vec![1, 3, 5, -1]) == false);
        assert!(triples_sum_to_zero(vec![1, 3, -2, 1]) == true);
        assert!(triples_sum_to_zero(vec![1, 2, 3, 7]) == false);
        assert!(triples_sum_to_zero(vec![1, 2, 5, 7]) == false);
        assert!(triples_sum_to_zero(vec![2, 4, -5, 3, 9, 7]) == true);
        assert!(triples_sum_to_zero(vec![1]) == false);
        assert!(triples_sum_to_zero(vec![1, 3, 5, -100]) == false);
        assert!(triples_sum_to_zero(vec![100, 3, 5, -100]) == false);
    }
}
