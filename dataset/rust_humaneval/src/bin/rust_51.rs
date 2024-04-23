fn main() {}

/*

    remove_vowels is a function that takes string and returns string without vowels.

*/

fn remove_vowels(text: &str) -> String {
    let vowels = "AEIOUaeiou";
    let mut out = String::new();
    for c in text.chars() {
        if !vowels.contains(c) {
            out.push(c);
        }
    }
    out
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
