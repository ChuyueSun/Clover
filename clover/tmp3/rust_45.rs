
fn triangle_area(a: i32, b: i32, c: i32) -> f64 {
    if a + b <= c || a + c <= b || b + c <= a {
        return -1.0;
    }

    let s = (a + b + c) as f64 / 2.0;
    let area = (s * (s - a as f64) * (s - b as f64) * (s - c as f64)).sqrt();
    (area * 100.0).round() / 100.0
}

fn main() {
    // Example usage:
    println!("{:.2}", triangle_area(3, 4, 5)); // Should output the area of a 3,4,5 triangle
    println!("{:.2}", triangle_area(1, 1, 2)); // Should output -1.0 as it doesn't form a valid triangle
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
