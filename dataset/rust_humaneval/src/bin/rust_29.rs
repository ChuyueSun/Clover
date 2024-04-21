fn main() {}

/*
 Filter an input list of strings only for ones that start with a given prefix.

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

fn filter_by_prefix(strings: Vec<String>, prefix: String) -> Vec<String> {
    return strings
        .into_iter()
        .filter(|s| s.starts_with(&prefix))
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_by_prefix() {
        let v_empty: Vec<String> = vec![];
        assert!(filter_by_prefix(vec![], "john".to_string()) == v_empty);
        assert!(
            filter_by_prefix(
                vec![
                    "xxx".to_string(),
                    "asd".to_string(),
                    "xxy".to_string(),
                    "john doe".to_string(),
                    "xxxAAA".to_string(),
                    "xxx".to_string()
                ],
                "xxx".to_string()
            ) == vec!["xxx", "xxxAAA", "xxx"]
        );
    }
}
