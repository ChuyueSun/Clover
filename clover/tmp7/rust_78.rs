
fn hex_key(num: &str) -> usize {
    num.chars()
        .filter(|c| match c {
            '2' | '3' | '5' | '7' | 'B' | 'D' => true,
            _ => false,
        })
        .count()
}

fn main() {
    // Example usage:
    let hex_num = "1A3B5D7";
    println!("Number of prime hex digits in {}: {}", hex_num, hex_key(hex_num));
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
