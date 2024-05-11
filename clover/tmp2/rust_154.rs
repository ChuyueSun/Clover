
fn cycpattern_check(a: &str, b: &str) -> bool {
    if b.is_empty() || b.len() > a.len() {
        return false;
    }

    let mut sb = b.to_string();
    for _ in 0..b.len() {
        if a.contains(&sb) {
            return true;
        }
        let ch = sb.remove(0);
        sb.push(ch);
    }
    
    false
}

fn main() {
    // Example usage
    let word1 = "exampleword";
    let word2 = "ordex";
    println!("Does '{}' contain '{}' or any of its rotations? {}", word1, word2, cycpattern_check(word1, word2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cycpattern_check() {
        assert_eq!(cycpattern_check("xyzw", "xyw"), false);
        assert_eq!(cycpattern_check("yello", "ell"), true);
        assert_eq!(cycpattern_check("whattup", "ptut"), false);
        assert_eq!(cycpattern_check("efef", "fee"), true);
        assert_eq!(cycpattern_check("abab", "aabb"), false);
        assert_eq!(cycpattern_check("winemtt", "tinem"), true);
    }

}
