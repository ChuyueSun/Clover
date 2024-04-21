
fn main(){ 
 } 

/*

    Given a string text, replace all spaces in it with underscores, 
    and if a string has more than 2 consecutive spaces, 
    then replace all consecutive spaces with - 
    
*/

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn fix_spaces(text: &str) -> String {


    let mut out = String::new();
    let mut spacelen = 0;
    for c in text.chars() {
        if c == ' ' {
            spacelen += 1;
        } else {
            if spacelen == 1 {
                out.push('_');
            }
            if spacelen == 2 {
                out.push_str("__");
            }
            if spacelen > 2 {
                out.push('-');
            }
            spacelen = 0;
            out.push(c);
        }
    }
    if spacelen == 1 {
        out.push('_');
    }
    if spacelen == 2 {
        out.push_str("__");
    }
    if spacelen > 2 {
        out.push('-');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fix_spaces() {
        assert_eq!(fix_spaces("Example"), "Example");
        assert_eq!(fix_spaces("Mudasir Hanif "), "Mudasir_Hanif_");
        assert_eq!(
            fix_spaces("Yellow Yellow  Dirty  Fellow"),
            "Yellow_Yellow__Dirty__Fellow"
        );
        assert_eq!(fix_spaces("Exa   mple"), "Exa-mple");
        assert_eq!(fix_spaces("   Exa 1 2 2 mple"), "-Exa_1_2_2_mple");
    }

}
