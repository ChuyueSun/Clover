
fn how_many_times(string: String, substring: String) -> usize {
    let mut count = 0;
    let substring_length = substring.len();

    // Loop through every starting point in the string
    for start_index in 0..string.len() {
        // Check if the remaining string is long enough for the substring
        if string.len() - start_index >= substring_length {
            // Check if the substring is found at the current position
            if &string[start_index..start_index + substring_length] == substring {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    // Example usage:
    let original_string = "ababa".to_string();
    let search_substring = "aba".to_string();
    let occurrences = how_many_times(original_string, search_substring);
    println!("The substring was found {} times.", occurrences);
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
