
fn string_xor(a: String, b: String) -> String {
    let mut result = String::new();
    
    let mut a_chars = a.chars().rev();
    let mut b_chars = b.chars().rev();

    loop {
        match (a_chars.next(), b_chars.next()) {
            (Some(a_char), Some(b_char)) => {
                if a_char != b_char {
                    result.push('1');
                } else {
                    result.push('0');
                }
            }
            (Some(a_char), None) => result.push(a_char),
            (None, Some(b_char)) => result.push(b_char),
            (None, None) => break,
        }
    }

    result.chars().rev().collect()
}

fn main() {
    // Example usage:
    let a = "1101".to_string();
    let b = "1011".to_string();
    println!("XOR: {}", string_xor(a, b));
}

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_string_xor() {
        assert!(string_xor("111000".to_string(), "101010".to_string()) == "010010");
        assert!(string_xor("1".to_string(), "1".to_string()) == "0");
        assert!(string_xor("0101".to_string(), "0000".to_string()) == "0101");
    }

}
