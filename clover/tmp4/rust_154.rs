
fn cycpattern_check(a: &str, b: &str) -> bool {
    for i in 0..b.len() {
        let first = &b[i..];
        let second = &b[..i];
        let rotation = first.to_string() + second;
        if a.contains(&rotation) {
            return true;
        }
    }
    false
}

fn main() {
    // Example usage:
    let a = "abcd";
    let b = "cdab";
    println!("{}", cycpattern_check(a, b)); // outputs: true

    let a = "rotation";
    let b = "tionrota";
    println!("{}", cycpattern_check(a, b)); // outputs: true

    let a = "hello";
    let b = "world";
    println!("{}", cycpattern_check(a, b)); // outputs: false
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
