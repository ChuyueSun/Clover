
fn how_many_times(string: String, substring: String) -> usize {
    let mut count = 0;
    let substring_len = substring.len();
    let mut possible_substring = string.as_str();
    
    while let Some(pos) = possible_substring.find(&substring) {
        count += 1;
        // move to the next character to find overlapping substrings
        possible_substring = &possible_substring[pos + 1..];
    }
    
    count
}

fn main() {
    let string = String::from("abcabcabc");
    let substring = String::from("abc");
    println!("The substring '{}' was found {} times in the string '{}'",  substring, how_many_times(string, substring), string);
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
