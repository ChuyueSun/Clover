
fn strlen(string: String) -> usize {
    string.len()
}

fn main() {
    let my_string = String::from("Hello, World!");
    println!("The length of the string is: {}", strlen(my_string));
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
