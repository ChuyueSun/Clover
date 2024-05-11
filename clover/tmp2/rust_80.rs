
fn is_happy(s: &str) -> bool {
    if s.len() < 3 {
        return false;
    }

    let chars: Vec<char> = s.chars().collect();

    for window in chars.windows(3) {
        if window[0] == window[1] || window[1] == window[2] || window[0] == window[2] {
            return false;
        }
    }

    true
}

fn main() {
    // Example usage:
    let s = "abcde";
    println!("Is the string '{}' happy? {}", s, is_happy(s)); // Expected: true
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
