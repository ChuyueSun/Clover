
fn encode_shift(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_uppercase() { 'A' } else { 'a' } as u8;
                // Assuming the encode_shift performed a forward shift by 3 according to the Caesar Cipher
                let decoded_char = (c as u8)
                    .checked_sub(3)
                    .or_else(|| Some(c as u8 + 26 - 3))
                    .map(|u| (u - base) % 26 + base)
                    .unwrap() as char;
                decoded_char
            } else {
                // Characters that are not in the English alphabet are returned as is
                c
            }
        })
        .collect()
}

// Example usage
fn main() {
    let encoded_string = "Khoor Zruog"; // "Hello World" shifted by 3 characters forward
    let decoded_string = encode_shift(&encoded_string);
    println!("Decoded string: {}", decoded_string);
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
