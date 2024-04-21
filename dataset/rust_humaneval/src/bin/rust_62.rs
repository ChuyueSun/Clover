fn main() {}

/*
 xs represent coefficients of a polynomial.
    xs[0] + xs[1] * x + xs[2] * x^2 + ....
     Return derivative of this polynomial in the same form.

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

fn derivative(xs: Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = vec![];
    for i in 1..xs.len() {
        res.push(i as i32 * xs.get(i).unwrap());
    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derivative() {
        assert!(derivative(vec![3, 1, 2, 4, 5]) == vec![1, 4, 12, 20]);
        assert!(derivative(vec![1, 2, 3]) == vec![2, 6]);
        assert!(derivative(vec![3, 2, 1]) == vec![2, 2]);
        assert!(derivative(vec![3, 2, 1, 0, 4]) == vec![2, 2, 0, 16]);
        assert!(derivative(vec![1]) == vec![]);
    }
}
