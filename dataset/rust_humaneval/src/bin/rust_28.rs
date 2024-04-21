
fn main(){ 
 } 

/*
 Concatenate list of strings into a single string
    
*/

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn concatenate(strings:Vec<String>) -> String{


    return strings.iter().fold(String::new(),|res: String, x:&String| res + &x.to_string());

}

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_concatenate() {
        assert!(concatenate(vec![]) == "".to_string());
        assert!(
            concatenate(vec!["x".to_string(), "y".to_string(), "z".to_string()])
                == "xyz".to_string()
        );
        assert!(
            concatenate(vec![
                "x".to_string(),
                "y".to_string(),
                "z".to_string(),
                "w".to_string(),
                "k".to_string()
            ]) == "xyzwk".to_string()
        );
    }


}
