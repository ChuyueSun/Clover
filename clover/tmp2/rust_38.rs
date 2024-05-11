
fn decode_cyclic(s: &str) -> String {
    let mut decoded_chars = Vec::new();
    let mut shift = 1;
    
    for c in s.chars().rev() {
        if let Some(decoded_char) = c as u32
            .checked_sub(shift)
            .and_then(std::char::from_u32)
        {
            decoded_chars.push(decoded_char);
        } else {
            // Handle wraparound for 'a' to 'z'
            let wraparound_char =
                std::char::from_u32('z' as u32 - ('a' as u32 - (c as u32 - shift + 1)) % 26).unwrap();
            decoded_chars.push(wraparound_char);
        }
        shift = (shift % 26) + 1;
    }

    decoded_chars.iter().rev().collect()
}
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


fn main() {
    // Example usage:
    let encoded = "ifmmp"; // "hello" encoded with encode_cyclic
    let decoded = decode_cyclic(&encoded);
    println!("Decoded string: {}", decoded); // Should print "hello"
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
