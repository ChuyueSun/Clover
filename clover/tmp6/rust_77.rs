
fn iscuber(a: i32) -> bool {
    let mut cuberoot = (a as f64).cbrt();
    cuberoot = cuberoot.round();
    let potential_cube = (cuberoot as i32).pow(3);
    a == potential_cube
}

fn main() {
    // Example test case
    let a = 27;
    println!("Is {} a perfect cube? {}", a, iscuber(a)); // Should print: Is 27 a perfect cube? true
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
