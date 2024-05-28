
use std::collections::HashSet;

fn count_distinct_characters(input_str: String) -> usize {
    let mut distinct_characters = HashSet::new();
    for c in input_str.chars() {
        distinct_characters.insert(c.to_ascii_lowercase());
    }
    distinct_characters.len()
}

fn main() {
    let input_str = String::from("Hello, World!");
    let count = count_distinct_characters(input_str);
    println!("Distinct characters: {}", count);
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