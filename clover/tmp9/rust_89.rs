
fn encrypt(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_alphabetic() {
                let base = if c.is_lowercase() { 'a' } else { 'A' } as u8;
                let offset = (c as u8 - base + 2 * 2) % 26;
                (base + offset) as char
            } else {
                c
            }
        })
        .collect()
}

fn main() {
    // Example usage:
    let encrypted = encrypt("Hello, World!");
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
