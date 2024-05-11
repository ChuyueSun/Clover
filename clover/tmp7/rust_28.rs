
fn concatenate(strings: Vec<String>) -> String {
    strings.join("")
}

fn main() {
    let words = vec!["Hello".to_string(), "World".to_string(), "!".to_string()];
    let sentence = concatenate(words);
    println!("{}", sentence);
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
