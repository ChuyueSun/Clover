
fn encode_shift(s: &str) -> String {
    let mut decoded = String::new();
    for char in s.chars() {
        if char.is_ascii_alphabetic() {
            // Determine if it's uppercase or lowercase for correct shifting
            let base = if char.is_ascii_uppercase() { 'A' } else { 'a' } as u8;
            // Calculate offset for lower and upper case separately
            let offset = (char as u8 + 26 - base - 3) % 26;
            // Shift character back by 3 and convert back to a char
            decoded.push((base + offset) as char);
        } else {
            // Non-alphabetic characters are pushed as-is
            decoded.push(char);
        }
    }
    decoded
}

fn main() {
    // Example usage:
    let encoded_string = "Khoor, Zruog!";
    let decoded_string = encode_shift(encoded_string);
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
