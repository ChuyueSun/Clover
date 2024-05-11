
fn encode(message: &str) -> String {
    let vowels = "aeiouAEIOU";
    let replacements = "cgkqCGKQ";
    message
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphabetic() {
                let offset = if ch.is_lowercase() { 32 } else { -32 };
                match vowels.find(ch) {
                    Some(index) => replacements.chars().nth(index).unwrap_or(ch),
                    None => char::from_u32((ch as u32) ^ 32).unwrap_or(ch),
                }
            } else {
                ch
            }
        })
        .collect()
}

fn main() {
    let message = "Hello World!";
    println!("Encoded: {}", encode(message));
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
