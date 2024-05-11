
fn change_base(x: i32, base: i32) -> String {
    if base <= 1 || base > 10 {
        panic!("Base must be between 2 and 9");
    }

    let mut result = Vec::new();
    let mut num = x;

    while num > 0 {
        result.push((num % base) as u8 + b'0');
        num /= base;
    }

    result.reverse();
    String::from_utf8(result).unwrap_or_else(|err| panic!("Invalid UTF-8 sequence: {}", err))
}

fn main() {
    // Example usage:
    println!("Base 2: {}", change_base(255, 2)); // Expected output: "11111111"
    println!("Base 3: {}", change_base(255, 3)); // Expected output: "100110"
    println!("Base 4: {}", change_base(255, 4)); // Expected output: "3333"
    // Add more examples if needed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change_base() {
        assert!(change_base(8, 3) == "22".to_string());
        assert!(change_base(9, 3) == "100".to_string());
        assert!(change_base(234, 2) == "11101010".to_string());
        assert!(change_base(16, 2) == "10000".to_string());
        assert!(change_base(8, 2) == "1000".to_string());
        assert!(change_base(7, 2) == "111".to_string());
    }

}
