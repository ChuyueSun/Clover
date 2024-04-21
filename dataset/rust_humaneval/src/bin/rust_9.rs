fn main() {}

/*
 From a given list of integers, generate a list of rolling maximum element found until given moment
    in the sequence.

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

fn rolling_max(numbers: Vec<i32>) -> Vec<i32> {
    let mut running_max: Option<i32> = None;
    let mut result: Vec<i32> = vec![];

    for n in numbers {
        if running_max == None {
            running_max = Some(n);
        } else {
            running_max = max(running_max, Some(n));
        }

        result.push(running_max.unwrap());
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rolling_max() {
        assert!(rolling_max(vec![]) == vec![]);
        assert!(rolling_max(vec![1, 2, 3, 4]) == vec![1, 2, 3, 4]);
        assert!(rolling_max(vec![4, 3, 2, 1]) == vec![4, 4, 4, 4]);
        assert!(rolling_max(vec![3, 2, 3, 100, 3]) == vec![3, 3, 3, 100, 100]);
    }
}
