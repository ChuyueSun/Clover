fn main() {}

/*
 Return length of given string

*/

fn strlen(strings: String) -> i32 {
    return strings.len() as i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strlen() {
        assert!(strlen("".to_string()) == 0);
        assert!(strlen("x".to_string()) == 1);
        assert!(strlen("asdasnakj".to_string()) == 9);
    }
}
