
fn decode_cyclic(encoded: &str) -> String {
    encoded
        .chars()
        .enumerate()
        .map(|(i, c)| {
            let shift = (i % 26) as u8;
            if c.is_ascii_lowercase() {
                ((c as u8 - 'a' as u8 + 26 - shift) % 26 + 'a' as u8) as char
            } else if c.is_ascii_uppercase() {
                ((c as u8 - 'A' as u8 + 26 - shift) % 26 + 'A' as u8) as char
            } else {
                c
            }
        })
        .collect()
}

fn main() {
    let encoded_string = "encoded example text";
    let decoded_string = decode_cyclic(encoded_string);
    println!("Decoded string: {}", decoded_string);
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
