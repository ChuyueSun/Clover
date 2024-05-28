
fn strlen(string: String) -> usize {
    string.chars().count()
}

fn main() {
    let example = String::from("Hello, World!");
    println!("The length of the string is: {}", strlen(example));
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