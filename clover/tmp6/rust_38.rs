
fn decode_cyclic(s: &str) -> String {
    let mut decoded_chars = Vec::new();
    let mut shift = 1;
    for c in s.chars().rev() {
        if let Some(decoded_char) = (c as i8 - shift as i8).checked_sub('a' as i8) {
            decoded_chars.push((decoded_char as u8 % 26 + b'a') as char);
        } else {
            // Handle char wrap around
            let wrap_around_char = (26 + (c as i8 - 'a' as i8) - shift) % 26 + b'a';
            decoded_chars.push(wrap_around_char as char);
        }
        shift = (shift + 1) % 26;
        if shift == 0 {
            shift = 1;
        }
    }
    decoded_chars.reverse();
    decoded_chars.iter().collect()
}

fn main() {
    // Example usage:
    let encoded_string = "bnchmf";
    let decoded_string = decode_cyclic(encoded_string);
    println!("Encoded string: {}, Decoded string: {}", encoded_string, decoded_string);
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
