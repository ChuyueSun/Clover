fn main() {}

/*

    You are given a positive integer n. You have to create an integer array a of length n.
        For each i (1 ≤ i ≤ n), the value of a[i] = i * i - i + 1.
        Return the number of triples (a[i], a[j], a[k]) of a where i < j < k,
    and a[i] + a[j] + a[k] is a multiple of 3.

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

fn get_matrix_triples(n: i32) -> i32 {
    let mut a = vec![];
    let mut sum = vec![vec![0, 0, 0]];
    let mut sum2 = vec![vec![0, 0, 0]];

    for i in 1..=n {
        a.push((i * i - i + 1) % 3);
        sum.push(sum[sum.len() - 1].clone());
        sum[i as usize][a[i as usize - 1] as usize] += 1;
    }

    for times in 1..3 {
        for i in 1..=n {
            sum2.push(sum2[sum2.len() - 1].clone());
            if i >= 1 {
                for j in 0..=2 {
                    sum2[i as usize][(a[i as usize - 1] + j) as usize % 3] +=
                        sum[i as usize - 1][j as usize];
                }
            }
        }
        sum = sum2.clone();
        sum2 = vec![vec![0, 0, 0]];
    }

    return sum[n as usize][0];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_matrix_triples() {
        assert_eq!(get_matrix_triples(5), 1);
        assert_eq!(get_matrix_triples(6), 4);
        assert_eq!(get_matrix_triples(10), 36);
        assert_eq!(get_matrix_triples(100), 53361);
    }
}
