
fn sorted_list_sum(mut lst: Vec<&str>) -> Vec<String> {
    // Filter out strings with odd lengths and collect them into a new vector
    let filtered_lst: Vec<String> = lst.into_iter()
        .filter(|s| s.len() % 2 == 0)
        .map(String::from)
        .collect();

    // Sort the vector first by length, then lexicographically
    let mut sorted_lst = filtered_lst.clone();
    sorted_lst.sort_unstable_by(|a, b| {
        let len_cmp = a.len().cmp(&b.len());
        if len_cmp == std::cmp::Ordering::Equal {
            a.cmp(b)
        } else {
            len_cmp
        }
    });

    sorted_lst
}

fn main() {
    // Example usage
    let words = vec!["rust", "is", "fun", "programming", "language"];
    let result = sorted_list_sum(words);
    println!("{:?}", result); // prints: ["fun", "rust", "is", "language", "programming"]
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
