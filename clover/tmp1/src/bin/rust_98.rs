
fn count_upper(s: &str) -> i32 {
    let vowels = ['A', 'E', 'I', 'O', 'U'];
    s.char_indices()
        .filter(|&(i, c)| i % 2 == 0 && vowels.contains(&c))
        .count() as i32
}

fn main() {
    let input_str = "AbcEfgIjkOmnpQrstUvwxyz";
    let count = count_upper(input_str);
    println!("The number of uppercase vowels at even indices: {}", count);
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
