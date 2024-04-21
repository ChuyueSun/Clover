
fn main(){ 
 } 

/*

    Given a positive integer n, return the count of the numbers of n-digit
    positive integers that start or end with 1.
    
*/

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn starts_one_ends(n:i32) -> i32{


    if n == 1 {
        return 1;
    };
    return 18 * i32::pow(10, (n - 2) as u32);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_starts_one_ends() {
        assert!(starts_one_ends(1) == 1);
        assert!(starts_one_ends(2) == 18);
        assert!(starts_one_ends(3) == 180);
        assert!(starts_one_ends(4) == 1800);
        assert!(starts_one_ends(5) == 18000);
    }

}
