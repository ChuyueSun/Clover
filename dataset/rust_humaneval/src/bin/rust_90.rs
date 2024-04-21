fn main() {}

/*

    You are given a list of integers.
    Write a function next_smallest() that returns the 2nd smallest element of the list.
    Return None if there is no such element.

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

fn next_smallest(lst: Vec<i32>) -> i32 {
    let mut res = 0;
    let mut lst_cp = lst.clone();
    let mut first: i32 = 0;
    let mut second: i32 = 0;

    if lst.iter().min() == None {
        res = -1;
    } else {
        if lst.iter().min() != None {
            first = *lst.iter().min().unwrap();
            let indx = lst.iter().position(|x| *x == first).unwrap();
            lst_cp.remove(indx);

            if lst_cp.iter().min() != None {
                second = *lst_cp.iter().min().unwrap();
            }
            if first != second {
                res = second;
            } else {
                res = -1;
            }
        }
    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_smallest() {
        assert!(next_smallest(vec![1, 2, 3, 4, 5]) == 2);
        assert!(next_smallest(vec![5, 1, 4, 3, 2]) == 2);
        assert!(next_smallest(vec![]) == -1);
        assert!(next_smallest(vec![1, 1]) == -1);
        assert!(next_smallest(vec![1, 1, 1, 1, 0]) == 1);
        assert!(next_smallest(vec![-35, 34, 12, -45]) == -35);
    }
}
