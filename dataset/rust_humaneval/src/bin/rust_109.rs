fn main() {}

/*
We have an array 'arr' of N integers arr[1], arr[2], ..., arr[N].The
    numbers in the array will be randomly ordered. Your task is to determine if
    it is possible to get an array sorted in non-decreasing order by performing
    the following operation on the given array:
        You are allowed to perform right shift operation any number of times.

    One right shift operation means shifting all elements of the array by one
    position in the right direction. The last element of the array will be moved to
    the starting position in the array i.e. 0th index.

    If it is possible to obtain the sorted array by performing the above operation
    then return True else return False.
    If the given array is empty then return True.

    Note: The given list is guaranteed to have unique elements.

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

fn move_one_ball(arr: Vec<i32>) -> bool {
    let mut num = 0;
    if arr.len() == 0 {
        return true;
    }
    for i in 1..arr.len() {
        if arr[i] < arr[i - 1] {
            num += 1;
        }
    }
    if arr[arr.len() - 1] > arr[0] {
        num += 1;
    }
    if num < 2 {
        return true;
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_one_ball() {
        assert!(move_one_ball(vec![3, 4, 5, 1, 2]) == true);
        assert!(move_one_ball(vec![3, 5, 10, 1, 2]) == true);
        assert!(move_one_ball(vec![4, 3, 1, 2]) == false);
        assert!(move_one_ball(vec![3, 5, 4, 1, 2]) == false);
        assert!(move_one_ball(vec![]) == true);
    }
}
