
const ALPHABET_LENGTH: usize = 26;

/// Decodes a string that was encoded with the encode_shift function.
/// For each alphabetical character, the encoding shifts it `n` positions in the alphabet
/// where `n` is the index of the character in the string, wrapping around the alphabet if necessary.
/// Non-alphabetical characters are not shifted.
///
/// # Arguments
///
/// * `s` - A string slice representing the encoded string.
///
/// # Returns
///
/// A `String` containing the decoded text.
fn decode_shift(s: &str) -> String {
    s.char_indices()
        .map(|(i, c)| {
            if c.is_ascii_alphabetic() {
                let shift = i % ALPHABET_LENGTH;
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                let c = c as u8;
                let shifted = if c >= base + shift {
                    c - shift as u8
                } else {
                    c + ALPHABET_LENGTH as u8 - shift as u8
                };
                shifted as char
            } else {
                c
            }
        })
        .collect()
}

fn main() {
    // Example usage:
    let encoded = "bdfhj"; // Encoded version of "abcde" with encode_shift
    let decoded = decode_shift(&encoded);
    println!("Decoded string: {}", decoded);
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
