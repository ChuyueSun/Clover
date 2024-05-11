
fn strlen(string: String) -> usize {
    string.len()
}

fn main() {
    let text = "Hello, World!".to_string();
    println!("Length of '{}': {}", text, strlen(text));
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
