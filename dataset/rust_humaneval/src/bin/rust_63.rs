
fn main(){ 
 } 

/*
The FibFib number sequence is a sequence similar to the Fibbonacci sequnece that's defined as follows:
    fibfib(0) == 0
    fibfib(1) == 0
    fibfib(2) == 1
    fibfib(n) == fibfib(n-1) + fibfib(n-2) + fibfib(n-3).
    Please write a function to efficiently compute the n-th element of the fibfib number sequence.
    
*/

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn fibfib(n:i32) -> i32{


    if n == 0 || n == 1{
        return 0;
    }
    if n == 2{
        return 1;
    }

    return fibfib(n-1) + fibfib(n-2) + fibfib(n-3);

}

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_fibfib() {
        assert!(fibfib(2) == 1);
        assert!(fibfib(1) == 0);
        assert!(fibfib(5) == 4);
        assert!(fibfib(8) == 24);
        assert!(fibfib(10) == 81);
        assert!(fibfib(12) == 274);
        assert!(fibfib(14) == 927);
    }

}
