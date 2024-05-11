
fn cycpattern_check(a: &str, b: &str) -> bool {
    if a.len() < b.len() {
        return false;
    }

    let bb = b.repeat(2);
    for i in 0..b.len() {
        let rot = &bb[i..i + b.len()];
        if a.contains(rot) {
            return true;
        }
    }

    false
}

fn main() {
    // Example usage:
    let word1 = "abcde";
    let word2 = "deabc";
    println!("Does the rotation of word2 appear in word1? {}", cycpattern_check(word1, word2)); // should print true
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
