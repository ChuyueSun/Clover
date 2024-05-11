
fn any_int(a: f64, b: f64, c: f64) -> bool {
    let a_int = a.fract() == 0.0;
    let b_int = b.fract() == 0.0;
    let c_int = c.fract() == 0.0;

    (a_int && b_int && c_int) && ((a == b + c) || (b == a + c) || (c == a + b))
}

fn main() {
    // Example usage:
    println!("{}", any_int(5.0, 3.0, 2.0)); // true
    println!("{}", any_int(5.0, 3.0, -2.0)); // false
    println!("{}", any_int(5.0, 3.0, 0.5)); // false
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
