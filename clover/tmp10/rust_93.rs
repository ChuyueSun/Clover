
fn encode(message: &str) -> String {
    message
        .chars()
        .map(|c| {
            if c.is_alphabetic() {
                let is_uppercase = c.is_uppercase();
                match c.to_ascii_lowercase() {
                    'a' => if is_uppercase { 'C' } else { 'c' }, 
                    'e' => if is_uppercase { 'G' } else { 'g' },
                    'i' => if is_uppercase { 'K' } else { 'k' },
                    'o' => if is_uppercase { 'Q' } else { 'q' },
                    'u' => if is_uppercase { 'W' } else { 'w' },
                    _ => if is_uppercase { c.to_ascii_lowercase() } else { c.to_ascii_uppercase() },
                }
            } else {
                c
            }
        })
        .collect()
}

fn main() {
    let message = "Hello World!";
    let encoded_message = encode(message);
    println!("{}", encoded_message); // Should print "jGQQK wQrMD!"
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
