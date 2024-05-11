
fn triangle_area_f64(a: f64, b: f64, c: f64) -> f64 {
    if a + b > c && a + c > b && b + c > a {
        let s = (a + b + c) / 2.0;
        let area = (s * (s - a) * (s - b) * (s - c)).sqrt();
        area.round()
    } else {
        -1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_triangle() {
        let area = triangle_area_f64(3.0, 4.0, 5.0);
        assert_eq!(area, 6.0);
    }

    #[test]
    fn test_invalid_triangle() {
        let area = triangle_area_f64(1.0, 2.0, 3.0);
        assert_eq!(area, -1.0);
    }

    #[test]
    fn test_decimal_triangle() {
        let area = triangle_area_f64(5.5, 5.5, 5.5);
        assert_eq!(area, 13.0);
    }
}

fn main() {
    // Example usage
    let area = triangle_area_f64(3.0, 4.0, 5.0);
    println!("The area of the triangle is: {}", area);
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
