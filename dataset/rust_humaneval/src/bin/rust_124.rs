fn main() {}

/*
You have to write a function which validates a given date string and
    returns True if the date is valid otherwise False.
    The date is valid if all of the following rules are satisfied:
    1. The date string is not empty.
    2. The number of days is not less than 1 or higher than 31 days for months 1,3,5,7,8,10,12. And the number of days is not less than 1 or higher than 30 days for months 4,6,9,11. And, the number of days is not less than 1 or higher than 29 for the month 2.
    3. The months should not be less than 1 or higher than 12.
    4. The date should be in the format: mm-dd-yyyy

*/

use md5;
use rand::Rng;
use regex::Regex;
use std::any::{Any, TypeId};
use std::{
    ascii::AsciiExt,
    cmp::{self, max},
    collections::{HashMap, HashSet},
    mem::replace,
    ops::Index,
    slice::Iter,
};

fn valid_date(date: &str) -> bool {
    let mut mm = 0;
    let mut dd = 0;
    let mut yy = 0;
    let mut i = 0;
    if date.len() != 10 {
        return false;
    }
    for i in 0..10 {
        if i == 2 || i == 5 {
            if date.chars().nth(i).unwrap() != '-' {
                return false;
            }
        } else if date.chars().nth(i).unwrap() < '0' || date.chars().nth(i).unwrap() > '9' {
            return false;
        }
    }
    mm = date[0..2].parse::<i32>().unwrap();
    dd = date[3..5].parse::<i32>().unwrap();
    yy = date[6..10].parse::<i32>().unwrap();
    if mm < 1 || mm > 12 {
        return false;
    }
    if dd < 1 || dd > 31 {
        return false;
    }
    if dd == 31 && (mm == 4 || mm == 6 || mm == 9 || mm == 11 || mm == 2) {
        return false;
    }
    if dd == 30 && mm == 2 {
        return false;
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_date() {
        assert_eq!(valid_date("03-11-2000"), true);
        assert_eq!(valid_date("15-01-2012"), false);
        assert_eq!(valid_date("04-0-2040"), false);
        assert_eq!(valid_date("06-04-2020"), true);
        assert_eq!(valid_date("01-01-2007"), true);
        assert_eq!(valid_date("03-32-2011"), false);
        assert_eq!(valid_date(""), false);
        assert_eq!(valid_date("04-31-3000"), false);
        assert_eq!(valid_date("06-06-2005"), true);
        assert_eq!(valid_date("21-31-2000"), false);
        assert_eq!(valid_date("04-12-2003"), true);
        assert_eq!(valid_date("04122003"), false);
        assert_eq!(valid_date("20030412"), false);
        assert_eq!(valid_date("2003-04"), false);
        assert_eq!(valid_date("2003-04-12"), false);
        assert_eq!(valid_date("04-2003"), false);
    }
}
