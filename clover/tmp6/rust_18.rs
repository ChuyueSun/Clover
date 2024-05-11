
fn how_many_times(string: String, substring: String) -> usize {
    if substring.is_empty() {
        return 0;
    }

    string.match_indices(&substring).count()
}

fn main() {
    // Example usage:
    let original_string = "ababa".to_string();
    let substring = "aba".to_string();

    let count = how_many_times(original_string, substring);
    println!("The substring was found {} times", count);
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn test_how_many_times() {
        assert!(how_many_times("".to_string(), "x".to_string()) == 0);
        assert!(how_many_times("xyxyxyx".to_string(), "x".to_string()) == 4);
        assert!(how_many_times("cacacacac".to_string(), "cac".to_string()) == 4);
        assert!(how_many_times("john doe".to_string(), "john".to_string()) == 1);
    }


}
