fn main() {}

/*
This function takes a list l and returns a list l' such that
    l' is identical to l in the odd indicies, while its values at the even indicies are equal
    to the values of the even indicies of l, but sorted.

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

fn sort_even(nmbs: Vec<i32>) -> Vec<i32> {
    let mut even = vec![];
    let mut out: Vec<i32> = vec![];

    for (indx, elem) in nmbs.iter().enumerate() {
        if indx % 2 == 0 {
            even.push(elem)
        }
    }
    even.sort();
    let mut indx_t: usize = 0;

    for i in 0..nmbs.len() {
        if i % 2 == 0 {
            if indx_t < even.len() {
                out.push(*even[indx_t]);
                indx_t += 1;
            }
        } else {
            out.push(nmbs[i]);
        }
    }
    return out;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_even() {
        assert_eq!(sort_even(vec![1, 2, 3]), vec![1, 2, 3]);
        assert_eq!(
            sort_even(vec![5, 3, -5, 2, -3, 3, 9, 0, 123, 1, -10]),
            vec![-10, 3, -5, 2, -3, 3, 5, 0, 9, 1, 123]
        );
        assert_eq!(
            sort_even(vec![5, 8, -12, 4, 23, 2, 3, 11, 12, -10]),
            vec![-12, 8, 3, 4, 5, 2, 12, 11, 23, -10]
        );
    }
}
