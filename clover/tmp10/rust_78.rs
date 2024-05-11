
fn hex_key(num: &str) -> u32 {
    let primes_in_hex = ['2', '3', '5', '7', 'B', 'D'];
    num.chars()
        .filter(|c| primes_in_hex.contains(c))
        .count() as u32
}

fn main() {
    // You can test the function here
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
