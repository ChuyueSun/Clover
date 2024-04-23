fn main() {}

/*

    takes as input string encoded with encode_cyclic function. Returns decoded string.

*/

fn decode_cyclic(s: &str) -> String {
    let l = s.len();
    let num = (l + 2) / 3;
    let mut output = String::new();
    for i in 0..num {
        let group = &s[i * 3..std::cmp::min(l, (i + 1) * 3)];
        // revert the cycle performed by the encode_cyclic function
        if group.len() == 3 {
            let x = format!("{}{}{}", &group[2..3], &group[0..1], &group[1..2]);
            output.push_str(&x);
        } else {
            output.push_str(group);
        }
    }
    output
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
