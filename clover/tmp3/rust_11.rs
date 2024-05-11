
fn string_xor(a: String, b: String) -> String {
    let mut result = String::new();
    let mut longer = a.chars();
    let mut shorter = b.chars();

    if a.len() < b.len() {
        longer = b.chars();
        shorter = a.chars();
    }

    for (i, char_long) in longer.enumerate() {
        if let Some(char_short) = shorter.next() {
            // Perform XOR on matching positions
            if char_long != char_short {
                result.push('1');
            } else {
                result.push('0');
            }
        } else {
            result.push_str(&longer.as_str()[i..]);
            break;
        }
    }

    result
}

fn main() {
    // Example usage:
    let a = "1101".to_string();
    let b = "0100".to_string();
    println!("{}", string_xor(a, b)); // Should print "1001"
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
