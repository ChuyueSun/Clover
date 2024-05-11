
fn encrypt(s: &str) -> String {
    let rotate_by = 2 * 2;
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let mut encrypted = String::new();

    for c in s.chars() {
        if let Some(idx) = alphabet.find(c.to_ascii_lowercase()) {
            let rotated_idx = (idx + rotate_by) % alphabet.len();
            if c.is_ascii_lowercase() {
                encrypted.push(alphabet.chars().nth(rotated_idx).unwrap());
            } else if c.is_ascii_uppercase() {
                encrypted.push(alphabet[rotated_idx..=rotated_idx].to_ascii_uppercase().chars().next().unwrap());
            }
        } else {
            encrypted.push(c);
        }
    }

    encrypted
}

fn main() {
    let message = "Encrypt this message!";
    let encrypted_message = encrypt(message);
    println!("Encrypted message: {:?}", encrypted_message);
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
