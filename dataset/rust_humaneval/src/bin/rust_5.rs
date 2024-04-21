
fn main(){ 
 } 

/*
 Insert a number 'delimeter' between every two consecutive elements of input list `numbers'
    
*/

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn intersperse(numbers:Vec<u32>, delimeter: u32) -> Vec<u32>{

    let mut res:Vec<u32> = vec![];
    numbers.iter().for_each(|item:&u32| {res.push(*item); res.push(delimeter);});
    res.pop();
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_intersperse() {
        assert!(intersperse(vec![], 7) == vec![]);
        assert!(intersperse(vec![5, 6, 3, 2], 8) == vec![5, 8, 6, 8, 3, 8, 2]);
        assert!(intersperse(vec![2, 2, 2], 2) == vec![2, 2, 2, 2, 2]);
    }

}
