
fn count_upper(s: &str) -> usize {
    s.char_indices()
        .filter(|(i, c)| {
            i % 2 == 0 && matches!(c, 'A' | 'E' | 'I' | 'O' | 'U')
        })
        .count()
}

fn main() {
    // Example usage:
    let input_str = "hEllo WorlD!";
    let count = count_upper(input_str);
    println!("The count of uppercase vowels at even indices is: {}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_upper() {
        assert!(count_upper("aBCdEf") == 1);
        assert!(count_upper("abcdefg") == 0);
        assert!(count_upper("dBBE") == 0);
        assert!(count_upper("B") == 0);
        assert!(count_upper("U") == 1);
        assert!(count_upper("") == 0);
        assert!(count_upper("EEEE") == 2);
    }

}
