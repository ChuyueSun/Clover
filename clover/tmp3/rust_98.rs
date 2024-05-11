
fn count_upper(s: &str) -> usize {
    const VOWELS: &[char] = &['A', 'E', 'I', 'O', 'U'];
    s.char_indices()
        .filter(|&(i, c)| i % 2 == 0 && VOWELS.contains(&c))
        .count()
}

fn main() {
    // Example usage:
    let input = "hEllo World! AEIOU byee";
    let count = count_upper(input);

    println!("The number of uppercase vowels in even indices: {}", count);
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
