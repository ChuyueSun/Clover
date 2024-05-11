
fn decode_cyclic(s: &str) -> String {
    let offset = (('z' as u8) - ('a' as u8)) as usize + 1;
    s.chars()
        .enumerate()
        .map(|(i, c)| {
            if c.is_ascii_lowercase() {
                let original_pos = (c as usize)
                    .wrapping_sub('a' as usize)
                    .wrapping_sub(i % offset)
                    % offset;
                (original_pos as u8 + 'a' as u8) as char
            } else {
                c
            }
        })
        .collect()
}

fn main() {
    // Example usage
    let encoded = "bdfhjlnprtvxz";
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
