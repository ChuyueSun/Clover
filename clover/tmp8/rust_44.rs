
fn change_base(x: i32, base: i32) -> String {
    assert!(base > 1 && base < 10, "base must be between 2 and 9");

    let mut result = Vec::new();
    let mut num = x;

    while num > 0 {
        result.push((num % base) as u8 + b'0');
        num /= base;
    }

    if result.is_empty() {
        result.push(b'0');
    }

    result.reverse();
    String::from_utf8(result).unwrap()
}

fn main() {
    let number = 255; // Example number
    let base = 2;     // Example base
    let converted = change_base(number, base);
    println!("Number {} in base {} is {}", number, base, converted);
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
