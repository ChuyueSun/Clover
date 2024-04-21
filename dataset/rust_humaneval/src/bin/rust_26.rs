fn main() {}

/*
 From a list of integers, remove all elements that occur more than once.
    Keep order of elements left the same as in the input.

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

fn remove_duplicates(numbers: Vec<i32>) -> Vec<i32> {
    let mut m: HashMap<i32, i32> = HashMap::new();

    for n in &numbers {
        *m.entry(*n).or_default() += 1;
    }
    let res: Vec<i32> = numbers
        .into_iter()
        .filter(|x| m.get(x) == Some(&1))
        .collect();
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        assert!(remove_duplicates(vec![]) == []);
        assert!(remove_duplicates(vec![1, 2, 3, 4]) == vec![1, 2, 3, 4]);
        assert!(remove_duplicates(vec![1, 2, 3, 2, 4, 3, 5]) == [1, 4, 5]);
    }
}
