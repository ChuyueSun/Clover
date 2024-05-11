
fn iscuber(a: i32) -> bool {
    let cube_root = (a as f64).cbrt();
    cube_root.round().powi(3) == a as f64
}

fn main() {
    // Example usage:
    let num = 27;
    println!("Is {} a cube of some integer? {}", num, iscuber(num));
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
