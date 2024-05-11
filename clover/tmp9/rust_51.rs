 the Rust program implementing the `remove_vowels` function.

```rust
fn remove_vowels(text: &str) -> String {
    text.chars()
        .filter(|c| !matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U'))
        .collect()
}

fn main() {
    let input = "Hello, World!";
    let result = remove_vowels(input);
    println!("{}", result); // Should print: "Hll, Wrld!"
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
