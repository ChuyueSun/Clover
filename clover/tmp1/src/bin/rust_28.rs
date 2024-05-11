
fn concatenate(strings: Vec<String>) -> String {
    strings.concat()
}

fn main() {
    let strings_to_concatenate = vec!["Hello, ".to_string(), "World!".to_string()];
    let result = concatenate(strings_to_concatenate);
    println!("Concatenated string: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_concatenate() {
        assert!(concatenate(vec![]) == "".to_string());
        assert!(
            concatenate(vec!["x".to_string(), "y".to_string(), "z".to_string()])
                == "xyz".to_string()
        );
        assert!(
            concatenate(vec![
                "x".to_string(),
                "y".to_string(),
                "z".to_string(),
                "w".to_string(),
                "k".to_string()
            ]) == "xyzwk".to_string()
        );
    }


}
