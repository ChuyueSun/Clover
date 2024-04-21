
fn main(){ 
 } 

/*
Complete the function that takes two integers and returns 
    the product of their unit digits.
    Assume the input is always valid.
    
*/

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn multiply(a:i32, b:i32) -> i32{


    return (i32::abs(a) % 10) * (i32::abs(b) % 10);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply() {
        assert!(multiply(148, 412) == 16);
        assert!(multiply(19, 28) == 72);
        assert!(multiply(2020, 1851) == 0);
        assert!(multiply(14, -15) == 20);
        assert!(multiply(76, 67) == 42);
        assert!(multiply(17, 27) == 49);
        assert!(multiply(0, 1) == 0);
        assert!(multiply(0, 0) == 0);
    }

}
