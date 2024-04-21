fn main() {}

/*
sum_to_n is a function that sums numbers from 1 to n.

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

fn sum_to_n(n: i32) -> i32 {
    n * (n + 1) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_to_n() {
        assert!(sum_to_n(1) == 1);
        assert!(sum_to_n(6) == 21);
        assert!(sum_to_n(11) == 66);
        assert!(sum_to_n(30) == 465);
        assert!(sum_to_n(100) == 5050);
    }
}
