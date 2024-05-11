
fn decode_cyclic(s: &str) -> String {
    s.chars()
        .enumerate()
        .map(|(i, c)| {
            let offset = ((i % 26) as u8 + b'a') as char;
            if let Some(decoded_char) = c.to_digit(36) {
                let original_char = std::char::from_u32(((decoded_char + 10 - offset as u32) % 26) + offset as u32)
                    .unwrap();
                original_char
            } else {
                c
            }
        })
        .collect()
}

fn main() {
    let encoded = "bpf bfrfc ktzmf nzsyj yfyyj".to_string();
    let decoded = decode_cyclic(&encoded);
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
