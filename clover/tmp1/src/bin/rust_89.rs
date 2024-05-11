
fn encrypt(s: &str) -> String {
    let rotate_by = 2 * 2; // rotate by twice multiplied to two places (4 in this case)
    let rotated = s
        .chars()
        .map(|c| match c {
            'a'..='z' => rotate_char(c, rotate_by, 'a', 'z'),
            'A'..='Z' => rotate_char(c, rotate_by, 'A', 'Z'),
            _ => c,
        })
        .collect();
    rotated
}

fn rotate_char(c: char, rotate_by: usize, start: char, end: char) -> char {
    let base = start as u8;
    let offset = (c as u8 - base + rotate_by as u8) % 26;
    (base + offset) as char
}

fn main() {
    let original = "The Quick Brown Fox Jumps Over The Lazy Dog!";
    let encrypted = encrypt(original);
    println!("Original: {}", original);
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
