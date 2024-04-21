
fn main(){ 
 } 

/*
sum_to_n is a function that sums numbers from 1 to n.
    
*/

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn sum_to_n(n: i32) -> i32 {


    n*(n+1)/2
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
