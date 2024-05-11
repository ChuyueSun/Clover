
fn count_upper(s: &str) -> usize {
    s.chars()
        .enumerate()
        .filter(|&(i, c)| i % 2 == 0 && "AEIOU".contains(c))
        .count()
}

fn main() {
    let s = "RuStProGrAmMiNg";
    println!("The number of uppercase vowels in even indices: {}", count_upper(s));
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
