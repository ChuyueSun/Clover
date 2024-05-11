
fn concatenate(strings: Vec<String>) -> String {
    strings.concat()
}

fn main() {
    let strings = vec![
        String::from("Hello"),
        String::from(", "),
        String::from("world"),
        String::from("!"),
    ];
    let result = concatenate(strings);
    println!("{}", result); // Output: Hello, world!
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
