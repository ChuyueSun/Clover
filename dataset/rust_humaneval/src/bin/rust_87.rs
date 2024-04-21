fn main() {}

/*

    You are given a 2 dimensional data, as a nested lists,
    which is similar to matrix, however, unlike matrices,
    each row may contain a different number of columns.
    Given lst, and integer x, find integers x in the list,
    and return list of tuples, [(x1, y1), (x2, y2) ...] such that
    each tuple is a coordinate - (row, columns), starting with 0.
    Sort coordinates initially by rows in ascending order.
    Also, sort coordinates of the row by columns in descending order.

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

fn get_row(lst: Vec<Vec<i32>>, x: i32) -> Vec<Vec<i32>> {
    let mut out: Vec<Vec<i32>> = vec![];
    for (indxi, elem1) in lst.iter().enumerate() {
        for (indxj, _) in elem1.iter().rev().enumerate() {
            if lst[indxi][indxj] == x {
                out.push(vec![indxi as i32, indxj as i32]);
            }
        }
    }
    return out;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_row() {
        assert!(
            get_row(
                vec![
                    vec![1, 2, 3, 4, 5, 6],
                    vec![1, 2, 3, 4, 1, 6],
                    vec![1, 2, 3, 4, 5, 1]
                ],
                1
            ) == vec![vec![0, 0], vec![1, 0], vec![1, 4], vec![2, 0], vec![2, 5]]
        );
        assert!(
            get_row(
                vec![
                    vec![1, 2, 3, 4, 5, 6],
                    vec![1, 2, 3, 4, 5, 6],
                    vec![1, 2, 3, 4, 5, 6],
                    vec![1, 2, 3, 4, 5, 6],
                    vec![1, 2, 3, 4, 5, 6],
                    vec![1, 2, 3, 4, 5, 6]
                ],
                2
            ) == vec![
                vec![0, 1],
                vec![1, 1],
                vec![2, 1],
                vec![3, 1],
                vec![4, 1],
                vec![5, 1]
            ]
        );
        assert!(
            get_row(
                vec![
                    vec![1, 2, 3, 4, 5, 6],
                    vec![1, 2, 3, 4, 5, 6],
                    vec![1, 1, 3, 4, 5, 6],
                    vec![1, 2, 1, 4, 5, 6],
                    vec![1, 2, 3, 1, 5, 6],
                    vec![1, 2, 3, 4, 1, 6],
                    vec![1, 2, 3, 4, 5, 1]
                ],
                1
            ) == vec![
                vec![0, 0],
                vec![1, 0],
                vec![2, 0],
                vec![2, 1],
                vec![3, 0],
                vec![3, 2],
                vec![4, 0],
                vec![4, 3],
                vec![5, 0],
                vec![5, 4],
                vec![6, 0],
                vec![6, 5]
            ]
        );
        let v: Vec<Vec<i32>> = vec![];
        assert!(get_row(vec![], 1) == v);
        assert!(get_row(vec![vec![1]], 2) == v);
        assert!(get_row(vec![vec![], vec![1], vec![1, 2, 3]], 3) == vec![vec![2, 2]]);
    }
}
