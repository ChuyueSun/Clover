
fn cycpattern_check(a: &str, b: &str) -> bool {
    if b.len() > a.len() {
        return false;
    }

    let rotation = b.repeat(2);
    a.contains(&rotation[..b.len()])
}

fn main() {
    let word1 = "abcde";
    let word2 = "deabc";
    println!("{}", cycpattern_check(word1, word2)); // Should output true
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
