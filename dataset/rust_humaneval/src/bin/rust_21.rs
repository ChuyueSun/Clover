fn main() {}

/*
 Given list of numbers (of at least two elements), apply a linear transform to that list,
    such that the smallest number will become 0 and the largest will become 1

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

fn rescale_to_unit(numbers: Vec<f32>) -> Vec<f32> {
    let min_number = *numbers
        .iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let max_number = *numbers
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    return numbers
        .iter()
        .map(|x: &f32| (x - min_number) / (max_number - min_number))
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rescale_to_unit() {
        assert!(rescale_to_unit(vec![2.0, 49.9]) == [0.0, 1.0]);
        assert!(rescale_to_unit(vec![100.0, 49.9]) == [1.0, 0.0]);
        assert!(rescale_to_unit(vec![1.0, 2.0, 3.0, 4.0, 5.0]) == [0.0, 0.25, 0.5, 0.75, 1.0]);
        assert!(rescale_to_unit(vec![2.0, 1.0, 5.0, 3.0, 4.0]) == [0.25, 0.0, 1.0, 0.5, 0.75]);
        assert!(rescale_to_unit(vec![12.0, 11.0, 15.0, 13.0, 14.0]) == [0.25, 0.0, 1.0, 0.5, 0.75]);
    }
}
