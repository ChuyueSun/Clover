
fn is_happy(s: &str) -> bool {
    if s.len() < 3 {
        return false;
    }

    let chars: Vec<char> = s.chars().collect();

    for i in 0..=chars.len() - 3 {
        if chars[i] == chars[i + 1] || chars[i + 1] == chars[i + 2] || chars[i] == chars[i + 2] {
            return false;
        }
    }

    true
}

fn main() {
    // Example usage
    let happy_str = "abcaxyz";
    let sad_str = "aabca";

    println!("Is '{}' a happy string? {}", happy_str, is_happy(happy_str)); // true
    println!("Is '{}' a happy string? {}", sad_str, is_happy(sad_str)); // false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_happy() {
        assert!(is_happy("a") == false);
        assert!(is_happy("aa") == false);
        assert!(is_happy("abcd") == true);
        assert!(is_happy("aabb") == false);
        assert!(is_happy("adb") == true);
        assert!(is_happy("xyy") == false);
        assert!(is_happy("iopaxpoi") == true);
        assert!(is_happy("iopaxioi") == false);
    }

}
