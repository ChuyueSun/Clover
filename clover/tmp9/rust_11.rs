
fn string_xor(a: String, b: String) -> String {
    let mut result = String::new();
    let bits_a = a.chars();
    let bits_b = b.chars();

    for (bit_a, bit_b) in bits_a.zip(bits_b) {
        if bit_a == bit_b {
            result.push('0');
        } else {
            result.push('1');
        }
    }

    result
}

fn main() {
    // Example usage:
    let a = "1101".to_string();
    let b = "1011".to_string();
    println!("{}", string_xor(a, b)); // Output should be "0110"
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
