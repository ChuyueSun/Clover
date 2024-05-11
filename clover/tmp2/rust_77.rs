
fn iscuber(a: i32) -> bool {
    let cubic_root = (a as f64).powf(1.0/3.0).round() as i32;
    cubic_root * cubic_root * cubic_root == a
}

fn main() {
    let test_number = 27;
    println!("Is {} a perfect cube? {}", test_number, iscuber(test_number));
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
