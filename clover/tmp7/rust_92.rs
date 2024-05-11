
fn any_int(a: f64, b: f64, c: f64) -> bool {
    (a == b + c || b == a + c || c == a + b) && a.fract() == 0.0 && b.fract() == 0.0 && c.fract() == 0.0
}

fn main() {
    println!("{}", any_int(5.0, 3.0, 2.0)); // Should print true
    println!("{}", any_int(5.1, 3.0, 2.0)); // Should print false, because 5.1 is not an integer
    println!("{}", any_int(10.0, 5.0, 5.0)); // Should print true
    println!("{}", any_int(1.0, 2.0, 3.0)); // Should print false, because no number is equal to the sum of the other two
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
