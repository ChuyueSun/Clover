
fn triangle_area(a: f64, b: f64, c: f64) -> f64 {
    if a + b > c && a + c > b && b + c > a {
        let s = (a + b + c) / 2.0;
        let area = (s * (s - a) * (s - b) * (s - c)).sqrt();
        area.rounding(2)
    } else {
        -1.0
    }
}

trait Rounding {
    fn rounding(&self, digits: u32) -> f64;
}

impl Rounding for f64 {
    fn rounding(&self, digits: u32) -> f64 {
        let multiplier = 10f64.powi(digits as i32);
        (self * multiplier).round() / multiplier
    }
}

fn main() {
    println!("{}", triangle_area(3.0, 4.0, 5.0)); // Example usage
}
```
Note: The function signature in your request does not match the description provided in the docstring. The docstring talks about a triangle formed by three sides, but the given signature only includes two parameters which suggest only one side and height are provided. The above program assumes a corrected function signature with three sides of a triangle (`a`, `b`, `c
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
