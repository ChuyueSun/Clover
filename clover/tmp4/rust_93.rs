
fn encode(message: &str) -> String {
    message
        .chars()
        .map(|c| {
            let is_upper = c.is_uppercase();
            let mut encoded_char = match c.to_ascii_lowercase() {
                'a' => 'c',
                'e' => 'g',
                'i' => 'k',
                'o' => 'q',
                'u' => 'w',
                _ => c,
            };

            if is_upper {
                encoded_char.make_ascii_uppercase();
            } else {
                encoded_char.make_ascii_lowercase();
            }

            if c.is_alphabetic() && c.is_lowercase() {
                encoded_char.to_ascii_uppercase()
            } else {
                encoded_char.to_ascii_lowercase()
            }
        })
        .collect()
}

fn main() {
    let message = "Hello, World!";
    let encoded = encode(message);
    println!("Encoded: {}", encoded);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        assert!(encode("TEST") == "tgst");
        assert!(encode("Mudasir") == "mWDCSKR");
        assert!(encode("YES") == "ygs");
        assert!(encode("This is a message") == "tHKS KS C MGSSCGG");
        assert!(encode("I DoNt KnOw WhAt tO WrItE") == "k dQnT kNqW wHcT Tq wRkTg");
    }

}
