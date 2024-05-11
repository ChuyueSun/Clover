
fn count_distinct_characters(input_str: String) -> usize {
    let mut chars_set = std::collections::HashSet::new();
    for character in input_str.to_lowercase().chars() {
        chars_set.insert(character);
    }
    chars_set.len()
}

fn main() {
    // Example usage:
    let example = "Hello, World!".to_string();
    println!("{}", count_distinct_characters(example)); // Should print the number of distinct lowercase characters
}

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_count_distinct_characters() {
        assert!(count_distinct_characters("".to_string()) == 0);
        assert!(count_distinct_characters("abcde".to_string()) == 5);
        assert!(
            count_distinct_characters(
                "abcde".to_string() + &"cade".to_string() + &"CADE".to_string()
            ) == 5
        );
        assert!(count_distinct_characters("aaaaAAAAaaaa".to_string()) == 1);
        assert!(count_distinct_characters("Jerry jERRY JeRRRY".to_string()) == 5);
    }

}
