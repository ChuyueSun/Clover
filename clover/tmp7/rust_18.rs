
fn how_many_times(string: String, substring: String) -> usize {
    let mut count = 0;
    let mut start = 0;
    let substring_len = substring.len();

    while let Some(position) = string[start..].find(&substring) {
        count += 1;
        start += position + 1;
    }

    count
}

fn main() {
    // Example usage:
    let original_string = String::from("abcabcabc");
    let search_substring = String::from("abc");
    let count = how_many_times(original_string, search_substring);
    println!("The substring was found {} times.", count);
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
