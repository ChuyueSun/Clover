
fn is_happy(s: &str) -> bool {
    if s.len() < 3 {
        return false;
    }

    let chars: Vec<char> = s.chars().collect();

    for i in 0..chars.len() - 2 {
        if chars[i] == chars[i + 1] || chars[i + 1] == chars[i + 2] || chars[i] == chars[i + 2] {
            return false;
        }
    }

    true
}

fn main() {
    // Test cases
    println!("{}", is_happy("abc")); // Should return true
    println!("{}", is_happy("aa")); // Should return false
    println!("{}", is_happy("abca")); // Should return false
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
