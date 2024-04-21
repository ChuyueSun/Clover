fn main() {}

/*

    Create a function that returns a tuple (a, b), where 'a' is
    the largest of negative integers, and 'b' is the smallest
    of positive integers in a list.
    If there is no negative or positive integers, return them as None.

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

fn largest_smallest_integers(lst: Vec<i32>) -> Vec<i32> {
    let mut maxneg = 0;
    let mut minpos = 0;
    for i in 0..lst.len() {
        if lst[i] < 0 && (maxneg == 0 || lst[i] > maxneg) {
            maxneg = lst[i];
        }
        if lst[i] > 0 && (minpos == 0 || lst[i] < minpos) {
            minpos = lst[i];
        }
    }
    vec![maxneg, minpos]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_smallest_integers() {
        assert_eq!(
            largest_smallest_integers(vec![2, 4, 1, 3, 5, 7]),
            vec![0, 1]
        );
        assert_eq!(
            largest_smallest_integers(vec![2, 4, 1, 3, 5, 7, 0]),
            vec![0, 1]
        );
        assert_eq!(
            largest_smallest_integers(vec![1, 3, 2, 4, 5, 6, -2]),
            vec![-2, 1]
        );
        assert_eq!(
            largest_smallest_integers(vec![4, 5, 3, 6, 2, 7, -7]),
            vec![-7, 2]
        );
        assert_eq!(
            largest_smallest_integers(vec![7, 3, 8, 4, 9, 2, 5, -9]),
            vec![-9, 2]
        );
        assert_eq!(largest_smallest_integers(vec![]), vec![0, 0]);
        assert_eq!(largest_smallest_integers(vec![0]), vec![0, 0]);
        assert_eq!(largest_smallest_integers(vec![-1, -3, -5, -6]), vec![-1, 0]);
        assert_eq!(
            largest_smallest_integers(vec![-1, -3, -5, -6, 0]),
            vec![-1, 0]
        );
        assert_eq!(
            largest_smallest_integers(vec![-6, -4, -4, -3, 1]),
            vec![-3, 1]
        );
        assert_eq!(
            largest_smallest_integers(vec![-6, -4, -4, -3, -100, 1]),
            vec![-3, 1]
        );
    }
}
