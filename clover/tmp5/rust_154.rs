
fn cycpattern_check(a: &str, b: &str) -> bool {
    let mut rotated_b = b.to_owned();
    
    for _ in 0..b.len() {
        if a.contains(&rotated_b) {
            return true;
        }
        
        // Left rotate the string by 1 character.
        rotated_b = rotated_b.chars().skip(1).chain(rotated_b.chars().take(1)).collect();
    }
    
    false
}

fn main() {
    // Example usage
    println!("{}", cycpattern_check("abcdef", "de")); // Returns true
    println!("{}", cycpattern_check("rotation", "tationr")); // Returns true
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
