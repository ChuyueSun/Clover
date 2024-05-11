
fn string_xor(a: String, b: String) -> String {
    format!("{:b}", usize::from_str_radix(&a, 2).unwrap() ^ usize::from_str_radix(&b, 2).unwrap())
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
