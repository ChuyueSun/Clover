
fn change_base(x: i32, base: i32) -> String {
    if base < 2 || base > 9 {
        panic!("Base must be between 2 and 9 inclusive");
    }
    let mut result = String::new();
    let mut num = x;
    while num > 0 {
        result.insert(0, std::char::from_digit((num % base) as u32, base as u32).unwrap());
        num /= base;
    }
    if result.is_empty() {
        result.push('0');
    }
    result
}

fn main() {
    let number = 255;
    let base = 8;
    
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
