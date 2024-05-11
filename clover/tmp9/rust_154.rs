
fn cycpattern_check(a: &str, b: &str) -> bool {
    let mut bb = b.repeat(2);
    bb.pop();
    a.contains(&bb)
}

fn main() {
    let a = "abcde";
    let b = "deabc";
    println!("{}", cycpattern_check(a, b)); // true
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
