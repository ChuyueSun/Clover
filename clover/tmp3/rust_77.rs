
fn iscuber(a: i32) -> bool {
    let cuberoot = (a as f64).cbrt();
    cuberoot.round() as i32 == cuberoot as i32
}

fn main() {
    let a = 27;
    println!("Is {} a cube of some integer? {}", a, iscuber(a));
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
