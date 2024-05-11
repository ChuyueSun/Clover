
fn encrypt(s: &str) -> String {
    let shift = 4; // Shift down by two multiplied by two places
    let a = 'a' as u8;
    let z = 'z' as u8;

    let encrypt_char = |c: char| -> char {
        if c.is_ascii_alphabetic() {
            let offset = if c.is_ascii_lowercase() { a } else { a - 32 };
            let rotated = ((c as u8 - offset + shift) % 26 + offset) as char;
            rotated
        } else {
            c
        }
    };

    s.chars().map(encrypt_char).collect()
}

fn main() {
    // Example usage:
    println!("{}", encrypt("hello")); // "lipps"
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
