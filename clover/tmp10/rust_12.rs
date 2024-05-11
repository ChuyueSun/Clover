
fn longest(strings: Vec<String>) -> Option<String> {
    if strings.is_empty() {
        return None;
    }

    let mut longest_string = &strings[0];

    for s in &strings[1..] {
        if s.len() > longest_string.len() {
            longest_string = s;
        }
    }

    Some(longest_string.to_string())
}

fn main() {
    // Example usage:
    let strings = vec![
        "short".to_string(),
        "longer".to_string(),
        "longest".to_string(),
        "sizeable".to_string(),
    ];
    println!("{:?}", longest(strings)); // Should print: Some("longest")
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
