
fn change_base(x: i32, base: i32) -> String {
    assert!(base > 1 && base < 10, "Base must be greater than 1 and less than 10.");
    
    let mut result = String::new();
    let mut n = x;

    while n > 0 {
        result = format!("{}{}", n % base, result);
        n /= base;
    }

    if result.is_empty() {
        result.push('0');
    }

    result
}

fn main() {
    println!("{}", change_base(10, 2)); // Example usage
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
