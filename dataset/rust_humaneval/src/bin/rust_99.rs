fn main() {}

/*

    Create a function that takes a value (string) representing a number
    and returns the closest integer to it. If the number is equidistant
    from two integers, round it away from zero.

    Note:
    Rounding away from zero means that if the given number is equidistant
    from two integers, the one you should return is the one that is the
    farthest from zero. For example closest_integer("14.5") should
    return 15 and closest_integer("-14.5") should return -15.

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

fn closest_integer(value: &str) -> i32 {
    return value.parse::<f64>().unwrap().round() as i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_closest_integer() {
        assert!(closest_integer("10") == 10);
        assert!(closest_integer("14.5") == 15);
        assert!(closest_integer("-15.5") == -16);
        assert!(closest_integer("15.3") == 15);
        assert!(closest_integer("0") == 0);
    }
}
