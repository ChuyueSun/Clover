
fn decode_cyclic(s: &str) -> String {
    let mut decoded_chars = Vec::new();
    let mut shift = 0;
    for c in s.chars().rev() {
        if let Some(decoded_char) = c as i32 - shift {
            if let Some(decoded_char) = std::char::from_u32(decoded_char as u32) {
                decoded_chars.push(decoded_char);
                shift = (shift + 1) % 26;
            } else {
                // Invalid character for decoding; skipping.
            }
        } else {
            // Invalid character shifting; skipping.
        }
    }
    decoded_chars.iter().rev().collect::<String>()
}

fn main() {
    // Example usage:
    let encoded_string = "ifmmp-!xpsme";
    let decoded_string = decode_cyclic(encoded_string);
    println!("Decoded string: {}", decoded_string); // Should print "hello world"
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
