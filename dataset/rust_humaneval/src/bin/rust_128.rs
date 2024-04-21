
fn main(){ 
 } 

/*

    You are given an array arr of integers and you need to return
    sum of magnitudes of integers multiplied by product of all signs
    of each number in the array, represented by 1, -1 or 0.
    Note: return None for empty arr.
    
*/

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn prod_signs(arr: Vec<i32>) -> i32 {


    if arr.is_empty() {
        return -32768;
    }
    let mut sum = 0;
    let mut prods = 1;
    for i in arr {
        sum += i.abs();
        if i == 0 {
            prods = 0;
        }
        if i < 0 {
            prods = -prods;
        }
    }
    sum * prods
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prod_signs() {
        assert_eq!(prod_signs(vec![1, 2, 2, -4]), -9);
        assert_eq!(prod_signs(vec![0, 1]), 0);
        assert_eq!(prod_signs(vec![1, 1, 1, 2, 3, -1, 1]), -10);
        assert_eq!(prod_signs(vec![]), -32768);
        assert_eq!(prod_signs(vec![2, 4, 1, 2, -1, -1, 9]), 20);
        assert_eq!(prod_signs(vec![-1, 1, -1, 1]), 4);
        assert_eq!(prod_signs(vec![-1, 1, 1, 1]), -4);
        assert_eq!(prod_signs(vec![-1, 1, 1, 0]), 0);
    }

}
