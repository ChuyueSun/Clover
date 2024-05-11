
fn cycpattern_check(a: &str, b: &str) -> bool {
    let b_len = b.len();
    
    for i in 0..b_len {
        let rotated = [&b[i..], &b[..i]].concat();
        if a.contains(&rotated) {
            return true;
        }
    }
    
    false
}

fn main() {
    println!("{}", cycpattern_check("hello", "llohe")); // true
    println!("{}", cycpattern_check("hello", "world")); // false
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
