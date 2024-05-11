
fn hex_prime_count(num: &str) -> usize {
    let primes_in_hex = ['2', '3', '5', '7', 'B', 'D'];
    num.chars()
       .filter(|c| primes_in_hex.contains(c))
       .count()
}

fn main() {
    let hex_number = "1A03B57D";
    let count = hex_prime_count(hex_number);
    println!("Number of prime hex digits: {}", count);
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
