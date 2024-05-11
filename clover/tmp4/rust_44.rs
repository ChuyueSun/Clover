
fn change_base(x: i32, base: i32) -> String {
    if base < 2 || base > 10 {
        panic!("Base must be between 2 and 10");
    }
    
    let mut result = String::new();
    let mut num = x;

    while num > 0 {
        result.insert(0, std::char::from_digit((num % base) as u32, base as u32).unwrap());
        num /= base;
    }

    if x == 0 { // Account for 0 as it won't be handled in the loop
        return "0".to_string();
    }

    result
}

fn main() {
    // Examples
    println!("Base 2 of 10: {}", change_base(10, 2)); // should be "1010"
    println!("Base 3 of 10: {}", change_base(10, 3)); // should be "101"
    println!("Base 4 of 10: {}", change_base(10, 4)); // should be "22"
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
