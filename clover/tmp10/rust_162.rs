
use md5::{Md5, Digest};
use std::fmt::{Formatter, Write};

fn string_to_md5(text: &str) -> Option<String> {
    if text.is_empty() {
        None
    } else {
        let mut hasher = Md5::new();
        hasher.update(text);
        let result = hasher.finalize();
        let mut md5_string = String::new();
        for byte in result {
            write!(&mut md5_string, "{:02x}", byte).expect("Unable to write");
        }
        Some(md5_string)
    }
}

fn main() {
    // example usage
    let text = "Hello, world!";
    let md5_hash = string_to_md5(text);
    println!("MD5 hash of '{}': {:?}", text, md5_hash);

    let empty_text = "";
    let md5_hash_empty = string_to_md5(empty_text);
    println!("MD5 hash of an empty string: {:?}", md5_hash_empty);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_to_md5() {
        assert_eq!(
            string_to_md5("Hello world"),
            "3e25960a79dbc69b674cd4ec67a72c62"
        );
        assert_eq!(string_to_md5(""), "None");
        assert_eq!(string_to_md5("A B C"), "0ef78513b0cb8cef12743f5aeb35f888");
        assert_eq!(
            string_to_md5("password"),
            "5f4dcc3b5aa765d61d8327deb882cf99"
        );
    }

}
