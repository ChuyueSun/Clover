
fn iscuber(a: i32) -> bool {
    let mut n = 0;
    while n.pow(3) < a {
        n += 1;
    }
    n.pow(3) == a
}

fn main() {
    // Example usage:
    println!("{}", iscuber(27)); // Should print 'true' as 27 is a cube of 3
    println!("{}", iscuber(28)); // Should print 'false' as 28 is not a perfect cube
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
