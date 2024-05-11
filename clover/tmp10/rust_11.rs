
fn string_xor(a: String, b: String) -> String {
    let mut result = String::new();
    let max_len = std::cmp::max(a.len(), b.len());
    let a = format!("{:0>width$}", a, width = max_len);
    let b = format!("{:0>width$}", b, width = max_len);

    for (char_a, char_b) in a.chars().zip(b.chars()) {
        match (char_a, char_b) {
            ('0', '0') | ('1', '1') => result.push('0'),
            ('0', '1') | ('1', '0') => result.push('1'),
            _ => unreachable!(),
        }
    }

    result
}

fn main() {
    let a = "1101".to_string();
    let b = "1001".to_string();
    let result = string_xor(a, b);
    println!("{}", result);
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
