
fn cycpattern_check(a: &str, b: &str) -> bool {
    let mut rotated = b.to_owned();
    for _ in 0..b.len() {
        if a.contains(&rotated) {
            return true;
        }
        rotated = rotated.chars().cycle().skip(1).take(b.len()).collect();
    }
    false
}

fn main() {
    // Example test cases
    println!("{}", cycpattern_check("abcd", "bc")); // true
    println!("{}", cycpattern_check("rotation", "tationr")); // true
    println!("{}", cycpattern_check("rotation", "rotator")); // false
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
