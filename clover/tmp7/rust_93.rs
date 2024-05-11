
fn encode(message: &str) -> String {
    message
        .chars()
        .map(|c| match c {
            'a' | 'A' => 'c',
            'e' | 'E' => 'g',
            'i' | 'I' => 'k',
            'o' | 'O' => 'q',
            'u' | 'U' => 'w',
            'c' | 'C' => 'a',
            'g' | 'G' => 'e',
            'k' | 'K' => 'i',
            'q' | 'Q' => 'o',
            'w' | 'W' => 'u',
            'a'..='z' => c.to_ascii_uppercase(),
            'A'..='Z' => c.to_ascii_lowercase(),
            _ => c,
        })
        .collect()
}

fn main() {
    let message = "Hello World!";
    let encoded = encode(&message);
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
