
fn encode(message: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    let mut encoded_message = String::new();

    for c in message.chars() {
        if vowels.contains(&c) {
            let offset = if c.is_lowercase() { 32 } else { -32 };
            let new_char = ((c as u8 + 2 - b'a' + 26) % 26 + b'a') as char;
            encoded_message.push((new_char as u8 + offset) as char);
        } else {
            encoded_message.push(if c.is_lowercase() {
                c.to_ascii_uppercase()
            } else {
                c.to_ascii_lowercase()
            });
        }
    }

    encoded_message
}

fn main() {
    // Example usage:
    let message = "Hello, World!";
    let encoded = encode(message);
    println!("Encoded message: {}", encoded);
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
