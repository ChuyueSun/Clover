
fn triangle_area(a: f64, b: f64, c: f64) -> f64 {
    if a + b > c && a + c > b && b + c > a {
        let s = (a + b + c) / 2.0;
        let area = (s * (s - a) * (s - b) * (s - c)).sqrt();
        area.round() + (area.fract() * 100.0).round() / 100.0
    } else {
        -1.0
    }
}

fn main() {
    // Example usage
    // Valid triangle sides
    println!("{}", triangle_area(3.0, 4.0, 5.0)); // Should return 6.0
    // Invalid triangle sides
    println!("{}", triangle_area(1.0, 1.0, 5.0)); // Should return -1.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangle_area() {
        assert!(triangle_area(5, 3) == 7.5);
        assert!(triangle_area(2, 2) == 2.0);
        assert!(triangle_area(10, 8) == 40.0);
    }

}
