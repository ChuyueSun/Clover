fn main() {}

/*
 Implement the function f that takes n as a parameter,
    and returns a list of size n, such that the value of the element at index i is the factorial of i if i is even
    or the sum of numbers from 1 to i otherwise.
    i starts from 1.
    the factorial of i is the multiplication of the numbers from 1 to i (1 * 2 * ... * i).

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

fn f(n: i32) -> Vec<i32> {
    let mut sum: i32 = 0;
    let mut prod: i32 = 1;
    let mut out: Vec<i32> = vec![];

    for i in 1..n + 1 {
        sum += i;
        prod *= i;

        if i % 2 == 0 {
            out.push(prod);
        } else {
            out.push(sum)
        };
    }
    return out;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f() {
        assert!(f(5) == vec![1, 2, 6, 24, 15]);
        assert!(f(7) == vec![1, 2, 6, 24, 15, 720, 28]);
        assert!(f(1) == vec![1]);
        assert!(f(3) == vec![1, 2, 6]);
    }
}
