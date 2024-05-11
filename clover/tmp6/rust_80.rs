
fn is_happy(s: &str) -> bool {
    if s.len() < 3 {
        return false;
    }
    s.as_bytes()
        .windows(3)
        .all(|window| window[0] != window[1] && window[0] != window[2] && window[1] != window[2])
}

fn main() {
    // Example usage:
    let happy_str = "abcde";
    println!("Is '{}' a happy string? {}", happy_str, is_happy(happy_str)); // Should print true

    let unhappy_str = "aab";
    println!("Is '{}' a happy string? {}", unhappy_str, is_happy(unhappy_str)); // Should print false
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
