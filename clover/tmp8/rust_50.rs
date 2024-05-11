
fn decode_shift(s: &str) -> String {
    // assuming the encode_shift function shifts every character by a fixed amount in the unicode scalar value
    // for simplicity, assume the shift value is a constant (like 1) used to encode
    const SHIFT_AMOUNT: u32 = 1;

    s.chars()
        .map(|c| {
            // only shift alphabetic characters and leave other characters unchanged
            if c.is_ascii_alphabetic() {
                let first_letter = if c.is_ascii_lowercase() { 'a' } else { 'A' } as u32;
                let shifted_pos = ((c as u32 - first_letter + 26 - SHIFT_AMOUNT) % 26) + first_letter;
                std::char::from_u32(shifted_pos).unwrap_or(c)
            } else {
                c
            }
        })
        .collect()
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
