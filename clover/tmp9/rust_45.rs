
fn triangle_area(a: i32, b: i32, c: i32) -> f64 {
    if a + b > c && a + c > b && b + c > a {
        // Using Heron's formula to calculate area of a triangle 
        // with sides a, b and c
        let s = (a + b + c) as f64 / 2.0;
        let area = (s * (s - a as f64) * (s - b as f64) * (s - c as f64)).sqrt();
        area.round() // Round to 2 decimal places
    } else {
        -1.0 // Invalid triangle sides
    }
}

// Example usage:
// let area = triangle_area(3, 4, 5);
// println!("The area of the triangle is: {}", area);

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
