
fn decode_shift(s: &str) -> String {
    // Create an empty String to store the decoded message
    let mut decoded = String::with_capacity(s.len());

    // Iterate over each character in the string
    for c in s.chars() {
        // Shift the character back by 1, considering wrapping from 'a' to 'z' and 'A' to 'Z'
        let shifted_char = match c {
            'a'..='z' => {
                if c > 'a' {
                    (c as u8 - 1) as char
                } else {
                    'z'
                }
            }
            'A'..='Z' => {
                if c > 'A' {
                    (c as u8 - 1) as char
                } else {
                    'Z'
                }
            }
            _ => c, // Non-alphabetic characters stay the same
        };

        // Append the shifted character to the decoded string
        decoded.push(shifted_char);
    }

    decoded
}

// The encode_shift function would presumably do the opposite of decode_shift,
// shifting letter characters forward by 1 in the alphabet, but it's not required
// to fulfill the given task and hence not implemented here.

fn main() {
    let encoded_message = "ifmmp xpsme"; // "hello world" shifted by 1
    let decoded_message = decode_shift(encoded_message);
    println!("Decoded message: {}", decoded_message);
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
