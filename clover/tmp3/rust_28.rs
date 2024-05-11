
fn concatenate(strings: Vec<String>) -> String {
    strings.concat()
}

fn main() {
    // Example usage:
    let words = vec![String::from("hello"), String::from("world")];
    let sentence = concatenate(words);
    println!("{}", sentence); // Output: helloworld
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
