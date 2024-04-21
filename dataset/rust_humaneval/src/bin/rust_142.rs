
fn main(){ 
 } 

/*
"
    This function will take a list of integers. For all entries in the list, the function shall square the integer entry if its index is a 
    multiple of 3 and will cube the integer entry if its index is a multiple of 4 and not a multiple of 3. The function will not 
    change the entries in the list whose indexes are not a multiple of 3 or 4. The function shall then return the sum of all entries. 
    
*/

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn sum_squares_142(lst: Vec<i32>) -> i32 {


    let mut sum = 0;
    for i in 0..lst.len() {
        if i % 3 == 0 {
            sum += lst[i] * lst[i];
        } else if i % 4 == 0 {
            sum += lst[i] * lst[i] * lst[i];
        } else {
            sum += lst[i];
        }
    }
    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_squares_142() {
        assert_eq!(sum_squares_142(vec![1, 2, 3]), 6);
        assert_eq!(sum_squares_142(vec![1, 4, 9]), 14);
        assert_eq!(sum_squares_142(vec![]), 0);
        assert_eq!(sum_squares_142(vec![1, 1, 1, 1, 1, 1, 1, 1, 1]), 9);
        assert_eq!(
            sum_squares_142(vec![-1, -1, -1, -1, -1, -1, -1, -1, -1]),
            -3
        );
        assert_eq!(sum_squares_142(vec![0]), 0);
        assert_eq!(sum_squares_142(vec![-1, -5, 2, -1, -5]), -126);
        assert_eq!(sum_squares_142(vec![-56, -99, 1, 0, -2]), 3030);
        assert_eq!(sum_squares_142(vec![-1, 0, 0, 0, 0, 0, 0, 0, -1]), 0);
        assert_eq!(
            sum_squares_142(vec![
                -16, -9, -2, 36, 36, 26, -20, 25, -40, 20, -4, 12, -26, 35, 37
            ]),
            -14196
        );
        assert_eq!(
            sum_squares_142(vec![
                -1, -3, 17, -1, -15, 13, -1, 14, -14, -12, -5, 14, -14, 6, 13, 11, 16, 16, 4, 10
            ]),
            -1448
        );
    }

}
