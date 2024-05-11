
fn change_base(x: i32, base: i32) -> String {
    assert!(base > 1 && base < 10, "base must be between 2 and 9 inclusive");
    
    let mut num = x;
    let mut result = String::new();
    
    while num > 0 {
        result = (num % base).to_string() + &result;
        num /= base;
    }

    if result.is_empty() {
        "0".to_string()
    } else {
        result
    }
}

fn main() {
    // Example usage:
    let number = 25;
    let base = 3;
    println!("Number {} in base {} is {}", number, base, change_base(number, base));
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
