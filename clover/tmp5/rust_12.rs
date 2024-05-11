
fn longest(strings: Vec<String>) -> Option<String> {
    strings.into_iter().max_by_key(|s| s.len())
}

fn main() {
    // Example usage
    let fruits = vec![
        "apple".to_string(),
        "banana".to_string(),
        "cherry".to_string(),
        "date".to_string(),
    ];

    if let Some(longest_fruit) = longest(fruits) {
        println!("The longest fruit is {}", longest_fruit);
    } else {
        println!("There are no fruits in the list!");
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
