
fn vowels_count(s: &str) -> usize {
    let mut count = 0;
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b'a' || item == b'e' || item == b'i' || item == b'o' || item == b'u' {
            count += 1;
        } else if item == b'y' && i == bytes.len() - 1 {
            count += 1;
        }
    }
    count
}

fn main() {
    println!("{}", vowels_count("hello")); // Example usage
    println!("{}", vowels_count("sky"));   // Example usage
}

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_vowels_count() {
        assert!(vowels_count("abcde") == 2);
        assert!(vowels_count("Alone") == 3);
        assert!(vowels_count("key") == 2);
        assert!(vowels_count("bye") == 1);
        assert!(vowels_count("keY") == 2);
        assert!(vowels_count("bYe") == 1);
        assert!(vowels_count("ACEDY") == 3);
    }

}
