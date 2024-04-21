fn main() {}

/*
The Fib4 number sequence is a sequence similar to the Fibbonacci sequnece that's defined as follows:
    fib4(0) -> 0
    fib4(1) -> 0
    fib4(2) -> 2
    fib4(3) -> 0
    fib4(n) -> fib4(n-1) + fib4(n-2) + fib4(n-3) + fib4(n-4).
    Please write a function to efficiently compute the n-th element of the fib4 number sequence.  Do not use recursion.

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

fn fib4(n: i32) -> i32 {
    let mut results: Vec<i32> = vec![0, 0, 2, 0];

    if n < 4 {
        return *results.get(n as usize).unwrap();
    }

    for _ in 4..n + 1 {
        results.push(
            results.get(results.len() - 1).unwrap()
                + results.get(results.len() - 2).unwrap()
                + results.get(results.len() - 3).unwrap()
                + results.get(results.len() - 4).unwrap(),
        );
        results.remove(0);
    }

    return *results.get(results.len() - 1).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib4() {
        assert!(fib4(5) == 4);
        assert!(fib4(8) == 28);
        assert!(fib4(10) == 104);
        assert!(fib4(12) == 386);
    }
}
