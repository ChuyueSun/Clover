
fn triangle_area_f64(a: f64, b: f64, c: f64) -> f64 {
    if a + b > c && a + c > b && b + c > a {
        let s = (a + b + c) / 2.0;
        let area = (s * (s - a) * (s - b) * (s - c)).sqrt();
        area.round_to(2)
    } else {
        -1.0
    }
}

trait RoundTo {
    fn round_to(self, decimal_places: u32) -> Self;
}

impl RoundTo for f64 {
    fn round_to(self, decimal_places: u32) -> f64 {
        let factor = 10f64.powi(decimal_places as i32);
        (self * factor).round() / factor
    }
}

fn main() {
    // Example usage:
    let area = triangle_area_f64(3.0, 4.0, 5.0);
    println!("Area of the triangle: {}", area);
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
