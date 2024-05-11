
/// Decodes a string encoded with the `encode_cyclic` function.
///
/// # Arguments
///
/// * `s` - A string slice that holds the encoded string.
///
/// # Returns
///
/// A `String` that contains the decoded string.
fn decode_cyclic(s: &str) -> String {
    s.chars()
        .enumerate()
        .map(|(i, c)| {
            let shift = (i % 26) as u8;
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                let byte_c = c as u8;
                let decoded_byte = if byte_c >= base + shift {
                    byte_c - shift
                } else {
                    byte_c + 26 - shift
                };
                decoded_byte as char
            } else {
                c
            }
        })
        .collect()
}
`
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
