
fn encode(message: &str) -> String {
    message.chars().map(|c| {
        // Check if c is a vowel and replace it by the character 2 places ahead.
        match c {
            'a' => 'c',
            'A' => 'C',

            'e' => 'g',
            'E' => 'G',

            'i' => 'k',
            'I' => 'K',

            'o' => 'q',
            'O' => 'Q',

            'u' => 'w',
            'U' => 'W',

            // If c is not a vowel, change case. Unicode supports case changes.
            c if c.is_ascii_alphabetic() => {
                if c.is_ascii_lowercase() {
                    c.to_ascii_uppercase()
                } else {
                    c.to_ascii_lowercase()
                }
            }

            // Leave other characters untouched.
            _ => c,
        }
    }).collect()
}

fn main() {
    // Example usage
    let message = "Hello World";
    let encoded_message = encode(message);
    println!("Encoded Message: {}", encoded_message);
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
