
fn cycpattern_check(a: &str, b: &str) -> bool {
    let double_b = b.repeat(2);
    double_b.contains(a) && b.len() == a.len()
}

fn main() {
    // You can test the function here
    let a = "abcde";
    let b = "cdeab";
    println!("Does '{}' contain a rotation of '{}'?: {}", a, b, cycpattern_check(a, b));
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
