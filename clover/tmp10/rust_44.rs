
fn change_base(x: i32, base: i32) -> String {
    if base < 2 || base > 10 {
        panic!("Base must be between 2 and 10");
    }
    let mut result = String::new();
    let mut num = x;
    
    while num > 0 {
        let digit = num % base;
        result = digit.to_string() + &result;
        num /= base;
    }
    
    if result.is_empty() {
        result = "0".to_string();
    }

    result
}

fn main() {
    let x = 255;
    let base = 8;
    let result = change_base(x, base);
    println!("{} in base {} is {}", x, base, result);
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
