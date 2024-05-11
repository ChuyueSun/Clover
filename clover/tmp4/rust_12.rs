
fn longest(strings: Vec<String>) -> Option<String> {
    if strings.is_empty() {
        None
    } else {
        let mut longest_string = &strings[0];
        for string in &strings[1..] {
            if string.len() > longest_string.len() {
                longest_string = string;
            }
        }
        Some(longest_string.clone())
    }
}

fn main() {
    let strings = vec!["short".to_string(), "longer".to_string(), "longeststringhere".to_string()];
    let result = longest(strings);
    println!("{:?}", result);
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
