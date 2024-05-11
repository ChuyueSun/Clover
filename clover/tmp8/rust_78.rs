
fn hex_key(num: &str) -> usize {
    let prime_hex_digits = ['2', '3', '5', '7', 'B', 'D'];
    num.chars()
        .filter(|&c| prime_hex_digits.contains(&c))
        .count()
}

fn main() {
    // Example usage:
    let hex_string = "1A3B5F7D9C";
    let count_prime_digits = hex_key(hex_string);
    println!("The number of prime hex digits is: {}", count_prime_digits);
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
