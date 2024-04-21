
fn main(){ 
 } 

/*
Return n-th Fibonacci number.
    
*/

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn fib(n:i32) -> i32{


    if n == 0{
        return 0;
    }
    if n == 1{
        return 1;
    }

    return fib(n-1) + fib(n-2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib() {
        assert!(fib(10) == 55);
        assert!(fib(1) == 1);
        assert!(fib(8) == 21);
        assert!(fib(11) == 89);
        assert!(fib(12) == 144);
    }

}
