fn main() {}

/*
 Return list of prime factors of given integer in the order from smallest to largest.
    Each of the factors should be listed number of times corresponding to how many times it appeares in factorization.
    Input number should be equal to the product of all factors

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

fn factorize(n: i32) -> Vec<i32> {
    let mut n = n;
    let mut factors = vec![];
    let mut divisor = 2;
    while divisor * divisor <= n {
        while n % divisor == 0 {
            factors.push(divisor);
            n = n / divisor;
        }
        divisor = divisor + 1;
    }
    if n > 1 {
        factors.push(n);
    }
    factors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorize() {
        assert_eq!(factorize(2), vec![2]);
        assert_eq!(factorize(4), vec![2, 2]);
        assert_eq!(factorize(8), vec![2, 2, 2]);
        assert_eq!(factorize(3 * 19), vec![3, 19]);
        assert_eq!(factorize(3 * 19 * 3 * 19), vec![3, 3, 19, 19]);
        assert_eq!(
            factorize(3 * 19 * 3 * 19 * 3 * 19),
            vec![3, 3, 3, 19, 19, 19]
        );
        assert_eq!(factorize(3 * 19 * 19 * 19), vec![3, 19, 19, 19]);
        assert_eq!(factorize(3 * 2 * 3), vec![2, 3, 3]);
    }
}
