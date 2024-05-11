
fn is_happy(s: &str) -> bool {
    if s.len() < 3 {
        return false;
    }
    for window in s.as_bytes().windows(3) {
        if window[0] == window[1] || window[1] == window[2] || window[0] == window[2] {
            return false;
        }
    }
    true
}

fn main() {
    // Example usage:
    let happy_str = "abcdef";
    println!("Is '{}' happy? {}", happy_str, is_happy(happy_str));

    let unhappy_str = "aba";
    println!("Is '{}' happy? {}", unhappy_str, is_happy(unhappy_str));
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
