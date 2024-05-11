
fn filter_by_substring(strings: Vec<String>, substring: &str) -> Vec<String> {
    strings.into_iter()
        .filter(|s| s.contains(substring))
        .collect()
}

fn main() {
    let strings = vec![
        String::from("apple"),
        String::from("banana"),
        String::from("apricot"),
        String::from("plum"),
    ];
    let substring = "ap";
    let filtered_strings = filter_by_substring(strings, substring);
    println!("{:?}", filtered_strings);
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
