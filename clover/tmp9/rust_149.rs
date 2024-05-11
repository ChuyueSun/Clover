
fn sorted_list_sum(mut lst: Vec<&str>) -> Vec<String> {
    // Remove strings with odd lengths
    lst.retain(|s| s.len() % 2 == 0);

    // Convert &str to String and sort the result
    let mut result: Vec<String> = lst.into_iter().map(String::from).collect();
    // Sort by length and then lexicographically
    result.sort_by(|a, b| match a.len().cmp(&b.len()) {
        std::cmp::Ordering::Equal => a.cmp(b),
        other => other,
    });

    result
}

fn main() {
    let words = vec!["rust", "coding", "in", "verus", "is", "fun", "challenge"];
    let sorted_list = sorted_list_sum(words);
    for word in sorted_list {
        println!("{}", word);
    }
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
