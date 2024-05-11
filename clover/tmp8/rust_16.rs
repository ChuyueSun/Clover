
fn count_distinct_characters(input_string: String) -> usize {
    let mut unique_chars = input_string
        .to_lowercase()
        .chars()
        .collect::<std::collections::HashSet<_>>();
    unique_chars.len()
}

fn main() {
    let input_string = "Example String".to_string();
    println!("Distinct characters: {}", count_distinct_characters(input_string));
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
