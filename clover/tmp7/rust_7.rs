
fn filter_by_substring(strings: Vec<String>, substring: &str) -> Vec<String> {
    strings
        .into_iter()
        .filter(|s| s.contains(substring))
        .collect()
}

fn main() {
    let strings = vec![
        "apple".to_string(),
        "banana".to_string(),
        "coconut".to_string(),
        "apricot".to_string(),
        "blueberry".to_string(),
    ];
    let filtered_strings = filter_by_substring(strings, "ap");
    for string in filtered_strings {
        println!("{}", string);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_filter_by_substring() {
        let v_empty: Vec<String> = vec![];
        assert!(filter_by_substring(vec![], String::from("john")) == v_empty);
        assert!(
            filter_by_substring(
                vec![
                    "xxx".to_string(),
                    "asd".to_string(),
                    "xxy".to_string(),
                    "john doe".to_string(),
                    "xxxAAA".to_string(),
                    "xxx".to_string()
                ],
                String::from("xxx")
            ) == vec!["xxx", "xxxAAA", "xxx"]
        );
        assert!(
            filter_by_substring(
                vec![
                    "xxx".to_string(),
                    "asd".to_string(),
                    "aaaxxy".to_string(),
                    "john doe".to_string(),
                    "xxxAAA".to_string(),
                    "xxx".to_string()
                ],
                String::from("xx")
            ) == vec!["xxx", "aaaxxy", "xxxAAA", "xxx"]
        );
        assert!(
            filter_by_substring(
                vec![
                    "grunt".to_string(),
                    "trumpet".to_string(),
                    "prune".to_string(),
                    "gruesome".to_string()
                ],
                String::from("run")
            ) == ["grunt", "prune"]
        );
    }

}
