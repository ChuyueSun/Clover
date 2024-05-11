
fn string_xor(a: String, b: String) -> String {
    let mut result = String::new();

    for (char_a, char_b) in a.chars().zip(b.chars()) {
        if char_a != char_b {
            result.push('1');
        } else {
            result.push('0');
        }
    }

    result
}

fn main() {
    let a = "1101".to_string();
    let b = "1001".to_string();
    println!("{}", string_xor(a, b)); // Should print "0100"
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
