fn main() {}

/*

    takes as input string encoded with encode_shift function. Returns decoded string.

*/

use rand::Rng;
use std::ascii::AsciiExt;

fn encode_shift(s: &str) -> String {
    let alphabet: Vec<&str> = vec![
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
        "s", "t", "u", "v", "w", "x", "y", "z",
    ];
    let mut output = String::new();

    for c in s.chars() {
        let mut lower = false;
        if c.is_ascii_lowercase() {
            lower = true;
        }
        let mut c_shift: String = "".to_string();
        if lower {
            let index: usize = alphabet.iter().position(|&x| x == c.to_string()).unwrap();
            c_shift = alphabet[(index + 5) % 26].to_string();
        } else {
            let c_lower: String = c.to_ascii_lowercase().to_string();
            let index: usize = alphabet.iter().position(|&x| x == c_lower).unwrap();
            c_shift = alphabet[(index + 5) % 26].to_string();
            c_shift = c_shift.to_ascii_uppercase().to_string();
        }

        output.push_str(&c_shift);
    }
    output
}

pub fn decode_shift(s: &str) -> String {
    let alphabet: Vec<&str> = vec![
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
        "s", "t", "u", "v", "w", "x", "y", "z",
    ];
    let mut output = String::new();

    for c in s.chars() {
        let mut lower = false;
        if c.is_ascii_lowercase() {
            lower = true;
        }
        let mut c_shift: String = "".to_string();
        if lower {
            let index: usize = alphabet.iter().position(|&x| x == c.to_string()).unwrap();
            c_shift = alphabet[((26 + (index as i32 - 5)) % 26) as usize].to_string();
        } else {
            let c_lower: String = c.to_ascii_lowercase().to_string();
            let index: usize = alphabet.iter().position(|&x| x == c_lower).unwrap();
            c_shift = alphabet[((26 + (index as i32 - 5)) % 26) as usize].to_string();
            c_shift = c_shift.to_ascii_uppercase().to_string();
        }

        output.push_str(&c_shift);
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    //Imposing that random characters that can be generated are solely from the alphabet
    fn test_decode_encode() {
        fn random_char() -> char {
            let mut rng = rand::thread_rng();
            let letter: char = match rng.gen_range(0, 2) {
                0 => rng.gen_range(b'a', b'z' + 1).into(),
                1 => rng.gen_range(b'A', b'Z' + 1).into(),
                _ => unreachable!(),
            };
            return letter;
        }

        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let r1: i32 = rng.gen();
            let l: i32 = 10 + r1 % 11;
            let mut str: String = "".to_string();

            for _ in 0..l {
                let chr: char = random_char();
                println!("{}", chr);
                str.push(chr);
            }

            let encoded_str: String = encode_shift(&str);
            assert!(decode_shift(&encoded_str) == str);
        }
    }
}
