
fn hex_key(num: &str) -> usize {
    let primes = ['2', '3', '5', '7', 'B', 'D'];
    num.chars().filter(|c| primes.contains(c)).count()
}

fn main() {
    // Example usage:
    let hex_str = "1A3B45D";
    println!("Number of prime hexadecimal digits: {}", hex_key(hex_str));
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
