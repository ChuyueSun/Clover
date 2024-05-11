
fn any_int(a: f64, b: f64, c: f64) -> bool {
    let are_integers = a.fract() == 0.0 && b.fract() == 0.0 && c.fract() == 0.0;
    are_integers && ((a == b + c) || (b == a + c) || (c == a + b))
}

fn main() {
    // Test cases to check the function `any_int`
    assert_eq!(any_int(1.0, 2.0, 3.0), true);
    assert_eq!(any_int(1.0, 2.5, 3.0), false);
    assert_eq!(any_int(1.0, 2.0, -1.0), true);
    assert_eq!(any_int(10.0, 5.0, 5.0), true);
    assert_eq!(any_int(1.0, 5.0, 3.0), false);

    // If you see this message without a panic, all tests passed!
    println!("All tests passed!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_any_int() {
        assert!(any_int(2.0, 3.0, 1.0) == true);
        assert!(any_int(2.5, 2.0, 3.0) == false);
        assert!(any_int(1.5, 5.0, 3.5) == false);
        assert!(any_int(2.0, 6.0, 2.0) == false);
        assert!(any_int(4.0, 2.0, 2.0) == true);
        assert!(any_int(2.2, 2.2, 2.2) == false);
        assert!(any_int(-4.0, 6.0, 2.0) == true);
        assert!(any_int(2.0, 1.0, 1.0) == true);
        assert!(any_int(3.0, 4.0, 7.0) == true);
        assert!(any_int(3.01, 4.0, 7.0) == false);
    }


}
