
use std::f64;

fn triangle_area(a: i32, b: i32, c: i32) -> f64 {
    if a + b <= c || a + c <= b || b + c <= a {
        return -1.0;
    }

    // Calculate semi-perimeter
    let s = (a + b + c) as f64 / 2.0;
    // Calculate the area using Heron's formula
    let area = (s * (s - a as f64) * (s - b as f64) * (s - c as f64)).sqrt();

    // Round to 2 decimal places
    (area * 100.0).round() / 100.0
}

fn main() {
    // Example usage:
    let area = triangle_area(3, 4, 5);
    println!("{:.2}", area); // Output should be 6.0
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
