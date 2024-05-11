
fn decode_shift(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let offset = match c.is_ascii_uppercase() {
                    true => 'A' as u8,
                    false => 'a' as u8,
                };
                let rotated = (c as u8 - offset - 3 + 26) % 26 + offset;
                rotated as char
            } else {
                c
            }
        })
        .collect()
}

fn main() {
    // Example usage:
    let encoded = "Khoor, Zruog!";
    let decoded = decode_shift(encoded);
    println!("Decoded string: {}", decoded); // Output: Hello, World!
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
