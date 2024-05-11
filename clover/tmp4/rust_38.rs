pub fn encode_cyclic(s: &str) -> String {
    // returns encoded string by cycling groups of three characters.
    // split string to groups. Each of length 3.
    let l = s.len();
    let num = (l + 2) / 3;
    let mut output = String::new();
    for i in 0..num {
        let group = &s[i * 3..std::cmp::min(l, (i + 1) * 3)];
        // cycle elements in each group. Unless group has fewer elements than 3.
        if group.len() == 3 {
            let x = format!("{}{}{}", &group[1..2], &group[2..3], &group[0..1]);
            output.push_str(&x);
        } else {
            output.push_str(group);
        }
    }
    output
}
fn decode_cyclic(s: &str) -> String {
    // The decode_cyclic function assumes that the encode_cyclic function encodes
    // each character by shifting it forward in the alphabet by its index (0-based)
    // modulo the alphabet size (assuming English alphabet size = 26)
    s.chars().enumerate()
        .map(|(i, c)| {
            // Calculate the original character by shifting back
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                // Wrapping sub to deal with underflow
                let original_char = ((c as u8)
                    .wrapping_sub((i % 26) as u8)
                    .wrapping_sub(base)
                    .wrapping_add(26) % 26
                    + base) as char;
                original_char
            } else {
                // Non-alphabetical characters are not encoded/decoded
                c
            }
        })
        .collect()
}

fn main() {
    // Example usage
    let encoded_str = "bcdefghijklmnopqrstuvwxyza";
    let decoded_str = decode_cyclic(encoded_str);
    println!("Decoded string: {}", decoded_str); // Should print "abcdefghijklmnopqrstuvwxyz" if the input was encoded by shifting each character one position further by its index.
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

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
