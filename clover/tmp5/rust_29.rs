
fn filter_by_prefix(strings: Vec<String>, prefix: String) -> Vec<String> {
    strings.into_iter().filter(|s| s.starts_with(&prefix)).collect()
}

fn main() {
    let strings = vec![
        "apple".to_string(),
        "banana".to_string(),
        "apricot".to_string(),
        "blueberry".to_string(),
    ];
    let prefix = "ap".to_string();

    let filtered_strings = filter_by_prefix(strings, prefix);
    for string in filtered_strings {
        println!("{}", string);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn test_filter_by_prefix() {
        let v_empty: Vec<String> = vec![];
        assert!(filter_by_prefix(vec![], "john".to_string()) == v_empty);
        assert!(
            filter_by_prefix(
                vec![
                    "xxx".to_string(),
                    "asd".to_string(),
                    "xxy".to_string(),
                    "john doe".to_string(),
                    "xxxAAA".to_string(),
                    "xxx".to_string()
                ],
                "xxx".to_string()
            ) == vec!["xxx", "xxxAAA", "xxx"]
        );
    }


}
