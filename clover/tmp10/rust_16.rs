
fn count_distinct_characters(input: String) -> usize {
    let mut chars = input.to_lowercase().chars().collect::<Vec<char>>();
    chars.sort_unstable();
    chars.dedup();
    chars.len()
}

fn main() {
    let example_string = String::from("Hello, World!");
    println!("Distinct characters: {}", count_distinct_characters(example_string));
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
