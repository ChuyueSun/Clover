
fn sorted_list_sum(mut lst: Vec<&str>) -> Vec<&str> {
    // Delete strings that have odd lengths
    lst.retain(|s| s.len() % 2 == 0);

    // Sort the list by length of each word, and then alphabetically for words of equal length
    lst.sort_unstable_by_key(|s| (s.len(), s.to_owned()));

    lst
}

fn main() {
    let words = vec!["apple", "pear", "banana", "kiwi", "melon"];
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
