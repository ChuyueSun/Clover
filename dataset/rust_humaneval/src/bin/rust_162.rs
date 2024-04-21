
fn main(){ 
 } 

/*

    Given a string 'text', return its md5 hash equivalent string.
    If 'text' is an empty string, return None.
    
*/

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn string_to_md5(text: &str) -> String {


    if text.is_empty() {
        return "None".to_string();
    }

    let digest = md5::compute(text.as_bytes());
    return format!("{:x}", digest);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_to_md5() {
        assert_eq!(
            string_to_md5("Hello world"),
            "3e25960a79dbc69b674cd4ec67a72c62"
        );
        assert_eq!(string_to_md5(""), "None");
        assert_eq!(string_to_md5("A B C"), "0ef78513b0cb8cef12743f5aeb35f888");
        assert_eq!(
            string_to_md5("password"),
            "5f4dcc3b5aa765d61d8327deb882cf99"
        );
    }

}
