
fn any_int(a: f64, b: f64, c: f64) -> bool {
    (a == b + c || b == a + c || c == a + b) && a.fract() == 0.0 && b.fract() == 0.0 && c.fract() == 0.0
}

fn main() {
    // Example usage
    println!("{}", any_int(8.0, 3.0, 5.0)); // should print true
    println!("{}", any_int(8.0, 3.1, 5.0)); // should print false
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
