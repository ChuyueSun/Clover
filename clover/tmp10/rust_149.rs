
fn sorted_list_sum(lst: Vec<&str>) -> Vec<String> {
    let mut even_length_words: Vec<String> = lst
        .into_iter()
        .filter(|s| s.len() % 2 == 0)
        .map(|s| s.to_string())
        .collect();
    
    even_length_words.sort_by(|a, b| {
        let len_cmp = a.len().cmp(&b.len());
        if len_cmp == std::cmp::Ordering::Equal {
            a.cmp(b)
        } else {
            len_cmp
        }
    });

    even_length_words
}

fn main() {
    // Example usage:
    let words = vec!["apple", "pear", "banana", "kiwi", "melon", "peach"];
    let sorted = sorted_list_sum(words);
    println!("{:?}", sorted); // ["kiwi", "pear", "apple", "banana", "melon", "peach"]
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
