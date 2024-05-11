
fn decode_cyclic(s: &str) -> String {
    s.chars()
        .enumerate()
        .map(|(i, c)| {
            let shift = (i % 26) as u8;
            if c.is_ascii_lowercase() {
                let a = b'a';
                ((c as u8 + 26 - shift - a) % 26 + a) as char
            } else if c.is_ascii_uppercase() {
                let a = b'A';
                ((c as u8 + 26 - shift - a) % 26 + a) as char
            } else {
                c
            }
        })
        .collect()
}

fn main() {
    // Example usage:
    let encoded = "bdfhjlnprtvxzBDFHJLNPRTVXZ";
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
