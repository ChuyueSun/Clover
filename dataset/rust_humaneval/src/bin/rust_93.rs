fn main() {}

/*

    Write a function that takes a message, and encodes in such a
    way that it swaps case of all letters, replaces all vowels in
    the message with the letter that appears 2 places ahead of that
    vowel in the english alphabet.
    Assume only letters.

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

fn encode(message: &str) -> String {
    let mut res: String = String::new();
    let v: Vec<char> = "aeiouAEIOU".to_string().chars().into_iter().collect();
    let d: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .to_string()
        .chars()
        .into_iter()
        .collect();

    for (indx, elem) in message.chars().into_iter().enumerate() {
        let mut c = elem.to_string();

        if v.contains(&elem) {
            let indx: usize = d.iter().position(|x| &elem == x).unwrap();
            c = d[indx + 2 as usize].to_string();
        }

        if elem.is_uppercase() {
            c = c.to_lowercase().to_string();
        } else {
            c = c.to_uppercase().to_string();
        }
        res.push_str(&c);
    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        assert!(encode("TEST") == "tgst");
        assert!(encode("Mudasir") == "mWDCSKR");
        assert!(encode("YES") == "ygs");
        assert!(encode("This is a message") == "tHKS KS C MGSSCGG");
        assert!(encode("I DoNt KnOw WhAt tO WrItE") == "k dQnT kNqW wHcT Tq wRkTg");
    }
}
