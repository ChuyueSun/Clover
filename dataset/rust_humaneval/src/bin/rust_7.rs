
fn main(){ 
 } 

/*
 Filter an input list of strings only for ones that contain given substring
    
*/

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn filter_by_substring(strings: Vec<String>, substring:String) -> Vec<String>{

    return strings.iter().filter(|x:&&String| x.contains(&substring)).map(String::from).collect();
}

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_filter_by_substring() {
        let v_empty: Vec<String> = vec![];
        assert!(filter_by_substring(vec![], String::from("john")) == v_empty);
        assert!(
            filter_by_substring(
                vec![
                    "xxx".to_string(),
                    "asd".to_string(),
                    "xxy".to_string(),
                    "john doe".to_string(),
                    "xxxAAA".to_string(),
                    "xxx".to_string()
                ],
                String::from("xxx")
            ) == vec!["xxx", "xxxAAA", "xxx"]
        );
        assert!(
            filter_by_substring(
                vec![
                    "xxx".to_string(),
                    "asd".to_string(),
                    "aaaxxy".to_string(),
                    "john doe".to_string(),
                    "xxxAAA".to_string(),
                    "xxx".to_string()
                ],
                String::from("xx")
            ) == vec!["xxx", "aaaxxy", "xxxAAA", "xxx"]
        );
        assert!(
            filter_by_substring(
                vec![
                    "grunt".to_string(),
                    "trumpet".to_string(),
                    "prune".to_string(),
                    "gruesome".to_string()
                ],
                String::from("run")
            ) == ["grunt", "prune"]
        );
    }

}
