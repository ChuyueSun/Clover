fn main() {}

/*
 brackets is a string of "<" and ">".
    return True if every opening bracket has a corresponding closing bracket.

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

fn correct_bracketing(bkts: &str) -> bool {
    let mut level: i32 = 0;

    for i in 0..bkts.len() {
        if bkts.chars().nth(i).unwrap() == '<' {
            level += 1;
        }

        if bkts.chars().nth(i).unwrap() == '>' {
            level -= 1;
        }

        if level < 0 {
            return false;
        }
    }
    if level != 0 {
        return false;
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correct_bracketing() {
        assert!(correct_bracketing("<>"));
        assert!(correct_bracketing("<<><>>"));
        assert!(correct_bracketing("<><><<><>><>"));
        assert!(correct_bracketing("<><><<<><><>><>><<><><<>>>"));
        assert!(!(correct_bracketing("<<<><>>>>")));
        assert!(!(correct_bracketing("><<>")));
        assert!(!(correct_bracketing("<")));
        assert!(!(correct_bracketing("<<<<")));
        assert!(!(correct_bracketing(">")));
        assert!(!(correct_bracketing("<<>")));
        assert!(!(correct_bracketing("<><><<><>><>><<>")));
        assert!(!(correct_bracketing("<><><<><>><>>><>")));
    }
}
