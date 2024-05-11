
fn encrypt(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_alphabetic() {
                let first_letter = if c.is_ascii_lowercase() { 'a' } else { 'A' };
                let offset = c as u8 - first_letter as u8;
                let new_offset = (offset + 2 * 2) % 26;
                (first_letter as u8 + new_offset) as char
            } else {
                c
            }
        })
        .collect()
}

fn main() {
    let encrypted = encrypt("hello");
    println!("Encrypted: {}", encrypted);
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
