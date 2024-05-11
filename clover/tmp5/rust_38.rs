
fn decode_cyclic(s: &str) -> String {
    let mut decoded = String::new();
    let mut shift = 1;

    for c in s.chars() {
        if let 'a'..='z' = c {
            let offset = c as u8 - 'a' as u8;
            let decoded_char = if offset >= shift {
                (c as u8 - shift) as char
            } else {
                (c as u8 + 26 - shift) as char
            };
            decoded.push(decoded_char);
        } else if let 'A'..='Z' = c {
            let offset = c as u8 - 'A' as u8;
            let decoded_char = if offset >= shift {
                (c as u8 - shift) as char
            } else {
                (c as u8 + 26 - shift) as char
            };
            decoded.push(decoded_char);
        } else {
            // Non-alphabetic characters are not encoded/decoded.
            decoded.push(c);
        }
        shift = (shift % 26) + 1;
    }

    decoded
}

fn main() {
    // Example usage:
    let encoded = "ifmmp xpsme"; // Encoded with encode_cyclic("hello world")
    let decoded = decode_cyclic(encoded);
    println!("Decoded string: {}", decoded);
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn test_decode_cyclic() {
        for _ in 0..100 {
            let l = 10 + rand::random::<u32>() % 11;
            let mut str = String::new();
            for _ in 0..l {
                let chr = 97 + rand::random::<u32>() % 26;
                str.push(chr as u8 as char);
            }
            let encoded_str = encode_cyclic(&str);
            assert_eq!(decode_cyclic(&encoded_str), str);
        }
    }

}
