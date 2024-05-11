
fn hex_key(num: &str) -> i32 {
    let mut count = 0;
    for digit in num.chars() {
        if matches!(digit, '2' | '3' | '5' | '7' | 'B' | 'D') {
            count += 1;
        }
    }
    count
}

fn main() {
    // Example usage:
    println!("{}", hex_key("1A2B3C4D5E6F7")); // Output should be 3
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_hex_key() {
        assert!(hex_key("AB") == 1);
        assert!(hex_key("1077E") == 2);
        assert!(hex_key("ABED1A33") == 4);
        assert!(hex_key("2020") == 2);
        assert!(hex_key("123456789ABCDEF0") == 6);
        assert!(hex_key("112233445566778899AABBCCDDEEFF00") == 12);
        assert!(hex_key("") == 0);
    }

}
