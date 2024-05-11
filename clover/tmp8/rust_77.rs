
fn iscuber(a: i32) -> bool {
    let cube_root = (a as f64).cbrt();
    cube_root.round() as i32 * cube_root.round() as i32 * cube_root.round() as i32 == a
}

fn main() {
    // Example test cases
    println!("{}", iscuber(27)); // true
    println!("{}", iscuber(28)); // false
    println!("{}", iscuber(-27)); // true
    println!("{}", iscuber(0)); // true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iscuber() {
        assert!(iscuber(1) == true);
        assert!(iscuber(2) == false);
        assert!(iscuber(-1) == true);
        assert!(iscuber(64) == true);
        assert!(iscuber(180) == false);
        assert!(iscuber(1000) == true);
        assert!(iscuber(0) == true);
        assert!(iscuber(1729) == false);
    }

}
