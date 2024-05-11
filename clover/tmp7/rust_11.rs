
fn string_xor(a: String, b: String) -> String {
    let mut result = String::new();
    let mut a_chars = a.chars().rev();
    let mut b_chars = b.chars().rev();

    loop {
        match (a_chars.next(), b_chars.next()) {
            (Some(x), Some(y)) => result.push(if x == y { '0' } else { '1' }),
            (Some(x), None) => result.push(x),
            (None, Some(y)) => result.push(y),
            (None, None) => break,
        }
    }

    result.chars().rev().collect()
}

fn main() {
    // Example usage:
    let a = "1010".to_string();
    let b = "1001".to_string();
    let result = string_xor(a, b);
    println!("The result of XOR is {}", result);
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
