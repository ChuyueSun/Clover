
fn main(){ 
 } 

/*

    Given a string s, count the number of uppercase vowels in even indices.
    
*/

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn count_upper(s:&str) -> i32 {


    let uvowel: &str = "AEIOU";
    let mut count: i32 = 0;

    for (indx, elem) in s.chars().into_iter().enumerate() {
        if indx % 2 == 0 {
            if uvowel.contains(elem) {
                count += 1;
            }
        }
    }
    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_upper() {
        assert!(count_upper("aBCdEf") == 1);
        assert!(count_upper("abcdefg") == 0);
        assert!(count_upper("dBBE") == 0);
        assert!(count_upper("B") == 0);
        assert!(count_upper("U") == 1);
        assert!(count_upper("") == 0);
        assert!(count_upper("EEEE") == 2);
    }

}
