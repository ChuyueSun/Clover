
use md5;
use std::option::Option;

fn string_to_md5(text: &str) -> Option<String> {
    if text.is_empty() {
        None
    } else {
        Some(format!("{:x}", md5::compute(text.as_bytes())))
    }
}

fn main() {
    let text = "hello";
    println!("{:?}", string_to_md5(text)); // Example usage

    let empty_string = "";
    println!("{:?}", string_to_md5(empty_string)); // Example usage with empty string
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
