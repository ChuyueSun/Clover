fn main() {}

/*
 For a given string, flip lowercase characters to uppercase and uppercase to lowercase.

*/

use std::ascii::AsciiExt;

pub fn flip_case(string: String) -> String {
    return string
        .chars()
        .into_iter()
        .fold(String::new(), |res: String, c: char| {
            if c.is_ascii_lowercase() {
                return res + &c.to_uppercase().to_string();
            } else {
                return res + &c.to_ascii_lowercase().to_string();
            }
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flip_case() {
        assert!(flip_case("".to_string()) == "".to_string());
        assert!(flip_case("Hello!".to_string()) == "hELLO!".to_string());
        assert!(
            flip_case("These violent delights have violent ends".to_string())
                == "tHESE VIOLENT DELIGHTS HAVE VIOLENT ENDS".to_string()
        );
    }
}
