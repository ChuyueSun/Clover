
fn string_xor(a: String, b: String) -> String {
    let mut xor_result = String::new();

    for (char_a, char_b) in a.chars().zip(b.chars()) {
        if char_a == char_b {
            xor_result.push('0');
        } else {
            xor_result.push('1');
        }
    }

    xor_result
}

fn main() {
    let a = "10101".to_string();
    let b = "00111".to_string();
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
