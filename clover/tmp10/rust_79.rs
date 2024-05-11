
fn decimal_to_binary(decimal: i32) -> String {
    format!("db{}db", format!("{:b}", decimal))
}

fn main() {
    // Example usage:
    let decimal = 10;
    println!("Binary representation: {}", decimal_to_binary(decimal));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decimal_to_binary() {
        assert!(decimal_to_binary(0) == "db0db".to_string());
        assert!(decimal_to_binary(32) == "db100000db".to_string());
        assert!(decimal_to_binary(103) == "db1100111db".to_string());
        assert!(decimal_to_binary(15) == "db1111db".to_string());
    }

}
