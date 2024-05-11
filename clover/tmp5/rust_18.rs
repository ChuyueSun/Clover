
fn how_many_times(string: String, substring: String) -> usize {
    let mut count = 0;
    let mut start = 0;
    
    while let Some(pos) = string[start..].find(&substring) {
        count += 1;
        start += pos + 1;
    }

    count
}

fn main() {
    // Example usage:
    let original_string = "ababa".to_string();
    let substring = "aba".to_string();
    let occurrences = how_many_times(original_string, substring);
    println!("The substring appears {} times", occurrences);
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
