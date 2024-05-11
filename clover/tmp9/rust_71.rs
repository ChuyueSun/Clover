
use std::f64;

fn triangle_area(a: f64, b: f64, c: f64) -> f64 {
    // Check if the sides can form a valid triangle
    if a + b > c && a + c > b && b + c > a {
        // Calculate the semi-perimeter
        let s = (a + b + c) / 2.0;
        // Calculate the area using Heron's formula
        let area = (s * (s - a) * (s - b) * (s - c)).sqrt();
        // Round the area to 2 decimal points
        (area * 100.0).round() / 100.0
    } else {
        // If it's not a valid triangle, return -1.0
        -1.0
    }
}

fn main() {
    // Example usage:
    println!("Area: {:?}", triangle_area(3.0, 4.0, 5.0)); // Should output Area: 6.0
    println!("Area: {:?}", triangle_area(1.0, 1.0, 5.0)); // Should output Area: -1.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangle_area_f64() {
        assert!(f64::abs(triangle_area_f64(3.0, 4.0, 5.0) - 6.00) < 0.01);
        assert!(f64::abs(triangle_area_f64(1.0, 2.0, 10.0) + 1.0) < 0.01);
        assert!(f64::abs(triangle_area_f64(4.0, 8.0, 5.0) - 8.18) < 0.01);
        assert!(f64::abs(triangle_area_f64(2.0, 2.0, 2.0) - 1.73) < 0.01);
        assert!(f64::abs(triangle_area_f64(1.0, 2.0, 3.0) + 1.0) < 0.01);
        assert!(f64::abs(triangle_area_f64(10.0, 5.0, 7.0) - 16.25) < 0.01);
        assert!(f64::abs(triangle_area_f64(2.0, 6.0, 3.0) + 1.0) < 0.01);
        assert!(f64::abs(triangle_area_f64(1.0, 1.0, 1.0) - 0.43) < 0.01);
        assert!(f64::abs(triangle_area_f64(2.0, 2.0, 10.0) + 1.0) < 0.01);
    }

}
