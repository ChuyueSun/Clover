
fn remove_vowels(text: &str) -> String {
    text.chars()
        .filter(|c| !matches!(c.to_lowercase().next().unwrap(), 'a' | 'e' | 'i' | 'o' | 'u'))
        .collect()
}

fn main() {
    // Example usage:
    let input = "Hello, World!";
    let output = remove_vowels(input);
    println!("{}", output); // Should print "Hll, Wrld!"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_vowels() {
        assert!(remove_vowels("") == "");
        assert!(remove_vowels("abcdef\nghijklm") == "bcdf\nghjklm");
        assert!(remove_vowels("fedcba") == "fdcb");
        assert!(remove_vowels("eeeee") == "");
        assert!(remove_vowels("acBAA") == "cB");
        assert!(remove_vowels("EcBOO") == "cB");
        assert!(remove_vowels("ybcd") == "ybcd");
    }

}
