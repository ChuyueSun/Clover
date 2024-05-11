
fn encode(message: &str) -> String {
    message
        .chars()
        .map(|c| {
            let is_uppercase = c.is_uppercase();
            let mut encoded_char = match c.to_ascii_lowercase() {
                'a' => 'c',
                'e' => 'g',
                'i' => 'k',
                'o' => 'q',
                'u' => 'w',
                other => other,
            };

            if is_uppercase {
                encoded_char = encoded_char.to_ascii_uppercase();
            } else {
                encoded_char = encoded_char.to_ascii_lowercase();
            }

            if c.is_alphabetic() {
                if is_uppercase {
                    encoded_char.to_ascii_lowercase()
                } else {
                    encoded_char.to_ascii_uppercase()
                }
            } else {
                encoded_char
            }
        })
        .collect()
}

fn main() {
    let message = "Hello World!";
    let encoded_message = encode(message);
    println!("{}", encoded_message); // should print "jgNNQ yqTNF!"
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
