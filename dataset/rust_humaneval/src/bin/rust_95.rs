fn main() {}

/*

    Given a dictionary, return True if all keys are strings in lower
    case or all keys are strings in upper case, else return False.
    The function should return False is the given dictionary is empty.

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

fn check_dict_case(dict: HashMap<&str, &str>) -> bool {
    if dict.is_empty() {
        return false;
    }
    let string_lower: fn(str: &str) -> bool = |str: &str| {
        return str.chars().into_iter().all(|c| c.is_ascii_lowercase());
    };
    let string_upper: fn(str: &str) -> bool = |str: &str| {
        return str.chars().into_iter().all(|c| c.is_ascii_uppercase());
    };

    let lower: bool = dict.keys().into_iter().all(|str| string_lower(str));
    let upper: bool = dict.keys().into_iter().all(|str| string_upper(str));
    return lower || upper;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_dict_case() {
        assert!(check_dict_case(HashMap::from([("p", "pineapple"), ("b", "banana")])) == true);
        assert!(
            check_dict_case(HashMap::from([
                ("p", "pineapple"),
                ("A", "banana"),
                ("B", "banana")
            ])) == false
        );
        assert!(
            check_dict_case(HashMap::from([
                ("p", "pineapple"),
                ("5", "banana"),
                ("a", "apple")
            ])) == false
        );
        assert!(
            check_dict_case(HashMap::from([
                ("Name", "John"),
                ("Age", "36"),
                ("City", "Houston")
            ])) == false
        );
        assert!(check_dict_case(HashMap::from([("STATE", "NC"), ("ZIP", "12345")])) == true);
        assert!(check_dict_case(HashMap::from([("fruit", "Orange"), ("taste", "Sweet")])) == true);
        assert!(check_dict_case(HashMap::new()) == false);
    }
}
