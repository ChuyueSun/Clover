
fn encode(message: &str) -> String {
    message
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let is_uppercase = c.is_uppercase();
                let shift_char = match c.to_ascii_lowercase() {
                    'a' => 'c',
                    'e' => 'g',
                    'i' => 'k',
                    'o' => 'q',
                    'u' => 'w',
                    'A' => 'C',
                    'E' => 'G',
                    'I' => 'K',
                    'O' => 'Q',
                    'U' => 'W',
                    _ => c,
                };
                if is_uppercase {
                    shift_char.to_ascii_uppercase()
                } else {
                    shift_char
                }
            } else {
                c
            }
        })
        .collect()
}

fn main() {
    let encoded = encode("Hello Rust Programmer!");
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
