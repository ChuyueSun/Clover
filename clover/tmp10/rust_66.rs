
fn digit_sum(s: &str) -> u32 {
    s.chars()
        .filter(|c| c.is_ascii_uppercase())
        .map(|c| c as u32)
        .sum()
}

fn main() {
    // Example usage:
    let input = "Hello World!";
    let result = digit_sum(input);
    println!("Sum of uppercase ASCII characters: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digitSum() {
        assert!(digitSum("") == 0);
        assert!(digitSum("abAB") == 131);
        assert!(digitSum("abcCd") == 67);
        assert!(digitSum("helloE") == 69);
        assert!(digitSum("woArBld") == 131);
        assert!(digitSum("aAaaaXa") == 153);
        assert!(digitSum(" How are yOu?") == 151);
        assert!(digitSum("You arE Very Smart") == 327);
    }


}
