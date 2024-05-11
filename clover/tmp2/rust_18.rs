
fn how_many_times(string: String, substring: String) -> i32 {
    if substring.is_empty() {
        return 0;
    }

    let mut count = 0;
    let mut start = 0;

    while let Some(pos) = string[start..].find(&substring) {
        count += 1;
        start += pos + 1;
    }

    count
}

fn main() {
    let text = "abcabcab".to_string();
    let sub = "abc".to_string();
    println!("Occurrences: {}", how_many_times(text, sub));
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
