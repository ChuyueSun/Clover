
fn cycpattern_check(a: &str, b: &str) -> bool {
    let b_len = b.len();
    let search_str = format!("{}{}", a, a);
    (0..b_len).any(|i| search_str.contains(&b[i..b_len].to_string() + &b[..i]))
}

fn main() {
    // Example usage
    let a = "abcde";
    let b = "cdeab";
    println!("{}", cycpattern_check(a, b)); // Should print "true"

    let a = "abcde";
    let b = "abced";
    println!("{}", cycpattern_check(a, b)); // Should print "false"
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
