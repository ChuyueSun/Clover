
fn triangle_area_f64(a: f64, b: f64, c: f64) -> f64 {
    if a + b > c && a + c > b && b + c > a {
        let s = (a + b + c) / 2.0;
        let area = (s * (s - a) * (s - b) * (s - c)).sqrt();
        area.rounded(2)
    } else {
        -1.0
    }
}

trait Precision {
    fn rounded(self, digits: u32) -> Self;
}

impl Precision for f64 {
    fn rounded(self, digits: u32) -> Self {
        let multiplier = 10f64.powi(digits as i32);
        (self * multiplier).round() / multiplier
    }
}

fn main() {
    println!("{:.2}", triangle_area_f64(3.0, 4.0, 5.0)); // Example usage
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
