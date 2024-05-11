
fn is_cuber(a: i32) -> bool {
    let cbrt_a: i32 = (a as f64).cbrt().round() as i32;
    cbrt_a.pow(3) == a
}

fn main() {
    // Example test cases
    println!("is 27 a cube? {}", is_cuber(27)); // true, since 3^3 = 27
    println!("is 20 a cube? {}", is_cuber(20)); // false, since there's no integer n such that n^3 = 20
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
