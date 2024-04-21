
fn main(){ 
 } 

/*

    Given an array of integers nums, find the minimum sum of any non-empty sub-array
    of nums.
    
*/

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn min_sub_array_sum(nums: Vec<i64>) -> i64 {


    let mut current = nums[0];
    let mut min = nums[0];
    for i in 1..nums.len() {
        if current < 0 {
            current = current + nums[i];
        } else {
            current = nums[i];
        }
        if current < min {
            min = current;
        }
    }
    min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_sub_array_sum() {
        assert!(min_sub_array_sum(vec![2, 3, 4, 1, 2, 4]) == 1);
        assert!(min_sub_array_sum(vec![-1, -2, -3]) == -6);
        assert!(min_sub_array_sum(vec![-1, -2, -3, 2, -10]) == -14);
        assert!(min_sub_array_sum(vec![-9999999999999999]) == -9999999999999999);
        assert!(min_sub_array_sum(vec![0, 10, 20, 1000000]) == 0);
        assert!(min_sub_array_sum(vec![-1, -2, -3, 10, -5]) == -6);
        assert!(min_sub_array_sum(vec![100, -1, -2, -3, 10, -5]) == -6);
        assert!(min_sub_array_sum(vec![10, 11, 13, 8, 3, 4]) == 3);
        assert!(min_sub_array_sum(vec![100, -33, 32, -1, 0, -2]) == -33);
        assert!(min_sub_array_sum(vec![-10]) == -10);
        assert!(min_sub_array_sum(vec![7]) == 7);
        assert!(min_sub_array_sum(vec![1, -1]) == -1);
    }

}
