
fn string_xor(a: String, b: String) -> String {
    let mut result = String::new();
    let chars_a = a.chars().rev();
    let chars_b = b.chars().rev();

    for (char_a, char_b) in chars_a.zip(chars_b) {
        match (char_a, char_b) {
            ('0', '0') | ('1', '1') => result.push('0'),
            ('0', '1') | ('1', '0') => result.push('1'),
            _ => unreachable!("Input strings should only consist of 1s and 0s"),
        }
    }

    result.chars().rev().collect()
}

fn main() {
    // Example usage:
    let a = "1100".to_string();
    let b = "1010".to_string();
    let xor_result = string_xor(a, b);
    println!("{}", xor_result); // Should print "0110"
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
