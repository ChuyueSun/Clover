
fn strlen(string: String) -> usize {
    string.len()
}

fn main() {
    let example = "example".to_string();
    println!("Length: {}", strlen(example));
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
