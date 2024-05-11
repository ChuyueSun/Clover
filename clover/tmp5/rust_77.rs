
fn iscuber(a: i32) -> bool {
    let cube_root = (a as f64).cbrt();
    let rounded = cube_root.round() as i32;
    rounded.pow(3) == a
}

fn main() {
    // You can test the function with some values
    println!("Is 27 a cube of some integer? {}", iscuber(27)); // Should print true
    println!("Is 28 a cube of some integer? {}", iscuber(28)); // Should print false
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
