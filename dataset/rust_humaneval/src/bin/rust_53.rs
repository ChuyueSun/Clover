fn main() {}

/*
Add two numbers x and y
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

fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert!(add(0, 1) == 1);
        assert!(add(1, 0) == 1);
        assert!(add(2, 3) == 5);
        assert!(add(5, 7) == 12);
        assert!(add(7, 5) == 12);
        for _ in 0..100 {
            let mut rng = rand::thread_rng();
            let mut x: i32 = rng.gen();
            x = x % 1000;
            let mut y: i32 = rng.gen();
            y = y % 1000;

            assert!(add(x, y) == x + y);
        }
    }
}
