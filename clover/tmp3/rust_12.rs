
fn longest(strings: Vec<String>) -> Option<String> {
    if strings.is_empty() {
        None
    } else {
        strings.into_iter().max_by_key(|s| s.len())
    }
}

fn main() {
    // Example Use
    let words = vec![
        "tiny".to_string(),
        "moderate".to_string(),
        "prolonged".to_string(),
        "lengthyword".to_string(),
    ];

    if let Some(longest_word) = longest(words) {
        println!("The longest word is: {}", longest_word);
    } else {
        println!("The input list is empty.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn test_longest() {
        assert!(longest(vec![]) == None);
        assert!(
            longest(vec!["x".to_string(), "y".to_string(), "z".to_string()])
                == Some("x".to_string())
        );
        assert!(
            longest(vec![
                "x".to_string(),
                "yyy".to_string(),
                "zzzz".to_string(),
                "www".to_string(),
                "kkkk".to_string(),
                "abc".to_string()
            ]) == Some("zzzz".to_string())
        );
    }

}
