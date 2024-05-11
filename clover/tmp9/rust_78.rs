
fn hex_prime_count(num: &str) -> u32 {
    let prime_hex_digits = ['2', '3', '5', '7', 'B', 'D'];
    num.chars()
        .filter(|&digit| prime_hex_digits.contains(&digit))
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_prime_count() {
        assert_eq!(hex_prime_count(""), 0);
        assert_eq!(hex_prime_count("123"), 2);
        assert_eq!(hex_prime_count("ABCD"), 2);
        assert_eq!(hex_prime_count("1"), 0);
        assert_eq!(hex_prime_count("2"), 1);
        assert_eq!(hex_prime_count("9AF"), 1);
        assert_eq!(hex_prime_count("789BDF"), 5);
    }
}

fn main() {
    // You can use this space to test the function with custom inputs if needed.
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
