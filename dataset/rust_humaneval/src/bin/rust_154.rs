fn main() {}

/*
You are given 2 words. You need to return True if the second word or any of its rotations is a substring in the first word

*/

fn cycpattern_check(a: &str, b: &str) -> bool {
    for i in 0..b.len() {
        let rotate = format!("{}{}", &b[i..], &b[..i]);
        if a.contains(&rotate) {
            return true;
        }
    }
    false
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
