 the Rust program that implements the functionality described in the docstring:

```rust
fn remove_vowels(text: &str) -> String {
    text.chars()
        .filter(|c| !"aeiouAEIOU".contains(*c))
        .collect()
}

fn main() {
    let input = "Hello, World!";
    println!("Original: {}", input);
    println!("No Vowels: {}", remove_vowels(input));
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
