
fn encode(message: &str) -> String {
    message
        .chars()
        .map(|c| match c {
            'a' | 'A' => 'c',
            'e' | 'E' => 'g',
            'i' | 'I' => 'k',
            'o' | 'O' => 'q',
            'u' | 'U' => 'w',
            'c' | 'C' => if c.is_uppercase() { 'a' } else { 'A' },
            'g' | 'G' => if c.is_uppercase() { 'e' } else { 'E' },
            'k' | 'K' => if c.is_uppercase() { 'i' } else { 'I' },
            'q' | 'Q' => if c.is_uppercase() { 'o' } else { 'O' },
            'w' | 'W' => if c.is_uppercase() { 'u' } else { 'U' },
            _ => if c.is_lowercase() {
                c.to_ascii_uppercase()
            } else {
                c.to_ascii_lowercase()
            },
        })
        .collect()
}

fn main() {
    let message = "Hello, World!";
    println!("Encoded message: {}", encode(message));
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
