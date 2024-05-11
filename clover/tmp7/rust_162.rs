
use md5;

fn string_to_md5(text: &str) -> Option<String> {
    if text.is_empty() {
        None
    } else {
        Some(format!("{:x}", md5::compute(text.as_bytes())))
    }
}

fn main() {
    // Example usage:
    if let Some(hash) = string_to_md5("hello") {
        println!("The MD5 hash of 'hello' is: {}", hash);
    } else {
        println!("No input to hash!");
    }

    if let Some(hash) = string_to_md5("") {
        println!("The MD5 hash of '' is: {}", hash);
    } else {
        println!("No input to hash!");
    }
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
