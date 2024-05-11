
fn encrypt(s: &str) -> String {
    const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";
    let shift = (2 * 2) % ALPHABET.len(); // Calculate shift based on alphabet length
    s.chars()
        .map(|c| {
            if c.is_alphabetic() {
                let is_uppercase = c.is_uppercase();
                let mut shifted_c = ALPHABET
                    .chars()
                    .cycle()
                    .skip_while(|&x| x != c.to_ascii_lowercase())
                    .nth(shift)
                    .unwrap_or(c); // Rotate the character

                if is_uppercase {
                    shifted_c = shifted_c.to_ascii_uppercase();
                }
                shifted_c
            } else {
                // Non-alphabetic characters remain unchanged
                c
            }
        })
        .collect()
}

fn main() {
    let text = "The quick brown fox jumps over the lazy dog";
    let encrypted = encrypt(text);
    println!("Encrypted text: {}", encrypted);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt() {
        assert!(encrypt("hi") == "lm");
        assert!(encrypt("asdfghjkl") == "ewhjklnop");
        assert!(encrypt("gf") == "kj");
        assert!(encrypt("et") == "ix");
        assert!(encrypt("faewfawefaewg") == "jeiajeaijeiak");
        assert!(encrypt("hellomyfriend") == "lippsqcjvmirh");
        assert!(
            encrypt("dxzdlmnilfuhmilufhlihufnmlimnufhlimnufhfucufh")
                == "hbdhpqrmpjylqmpyjlpmlyjrqpmqryjlpmqryjljygyjl"
        );
        assert!(encrypt("a") == "e");
    }

}
