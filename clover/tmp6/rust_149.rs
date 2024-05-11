
fn sorted_list_sum(lst: Vec<&str>) -> Vec<String> {
    let mut filtered_lst: Vec<String> = lst
        .into_iter()
        .filter(|s| s.len() % 2 == 0)
        .map(|s| s.to_string())
        .collect();
        
    filtered_lst.sort_unstable_by(|a, b| {
        a.len().cmp(&b.len()).then_with(|| a.cmp(b))
    });
    
    filtered_lst
}

fn main() {
    let words = vec!["even", "lengths", "stay", "odd", "out"];
    let sorted_words = sorted_list_sum(words);
    println!("{:?}", sorted_words);
}

#[cfg(test)]
mod tests {
    use super::*;

   #[test]
    fn test_sorted_list_sum() {
        assert_eq!(sorted_list_sum(vec!["aa", "a", "aaa"]), vec!["aa"]);
        assert_eq!(
            sorted_list_sum(vec!["school", "AI", "asdf", "b"]),
            vec!["AI", "asdf", "school"]
        );
        let v_empty: Vec<&str> = vec![];
        assert_eq!(sorted_list_sum(vec!["d", "b", "c", "a"]), v_empty);
        assert_eq!(
            sorted_list_sum(vec!["d", "dcba", "abcd", "a"]),
            vec!["abcd", "dcba"]
        );
        assert_eq!(
            sorted_list_sum(vec!["AI", "ai", "au"]),
            vec!["AI", "ai", "au"]
        );
        assert_eq!(sorted_list_sum(vec!["a", "b", "b", "c", "c", "a"]), v_empty);
        assert_eq!(
            sorted_list_sum(vec!["aaaa", "bbbb", "dd", "cc"]),
            vec!["cc", "dd", "aaaa", "bbbb"]
        );
    }

}
