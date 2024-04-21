fn main() {}

/*

    Given a list of numbers, return whether or not they are sorted
    in ascending order. If list has more than 1 duplicate of the same
    number, return False. Assume no negative numbers and only integers.

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

fn is_sorted(lst: Vec<i32>) -> bool {
    for i in 1..lst.len() {
        if lst[i] < lst[i - 1] {
            return false;
        }
        if i >= 2 && lst[i] == lst[i - 1] && lst[i] == lst[i - 2] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_sorted() {
        assert_eq!(is_sorted(vec![5]), true);
        assert_eq!(is_sorted(vec![1, 2, 3, 4, 5]), true);
        assert_eq!(is_sorted(vec![1, 3, 2, 4, 5]), false);
        assert_eq!(is_sorted(vec![1, 2, 3, 4, 5, 6]), true);
        assert_eq!(is_sorted(vec![1, 2, 3, 4, 5, 6, 7]), true);
        assert_eq!(is_sorted(vec![1, 3, 2, 4, 5, 6, 7]), false);
        assert_eq!(is_sorted(vec![]), true);
        assert_eq!(is_sorted(vec![1]), true);
        assert_eq!(is_sorted(vec![3, 2, 1]), false);
        assert_eq!(is_sorted(vec![1, 2, 2, 2, 3, 4]), false);
        assert_eq!(is_sorted(vec![1, 2, 3, 3, 3, 4]), false);
        assert_eq!(is_sorted(vec![1, 2, 2, 3, 3, 4]), true);
        assert_eq!(is_sorted(vec![1, 2, 3, 4]), true);
    }
}
