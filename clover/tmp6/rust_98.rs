
fn count_upper(s: &str) -> usize {
    s.chars()
        .enumerate()
        .filter(|(index, ch)| {
            index % 2 == 0 && "AEIOU".contains(*ch)
        })
        .count()
}

fn main() {
    let test_str = "hElLo WoRld AEIOU";
    println!("Number of uppercase vowels at even indices: {}", count_upper(test_str));
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
