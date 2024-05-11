
fn strlen(strings: String) -> usize {
    strings.len()
}

fn main() {
    let example_string = "Hello, World!".to_string();
    let length = strlen(example_string);
    println!("Length of the string is: {}", length);
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
