
fn count_upper(s: &str) -> usize {
    s.char_indices()
        .filter(|&(i, c)| i % 2 == 0 && "AEIOU".contains(c))
        .count()
}

fn main() {
    // Example usage:
    let test_str = "hEllo World, ThIs Is A tEsT";
    let uppercase_vowel_count = count_upper(test_str);
    println!("The number of uppercase vowels at even indices: {}", uppercase_vowel_count);
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
