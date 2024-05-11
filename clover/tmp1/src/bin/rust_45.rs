
fn triangle_area(a: i32, h: i32) -> f64 {
    if a <= 0 || h <= 0 {
        return -1.0;
    }
    let area = 0.5 * a as f64 * h as f64;
    (area * 100.0).round() / 100.0
}

fn main() {
    // Example usage:
    let a = 3;
    let h = 4;
    println!("Area of the triangle: {}", triangle_area(a, h));
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
