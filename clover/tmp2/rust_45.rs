
fn triangle_area(a: f64, b: f64, c: f64) -> f64 {
    if a + b > c && a + c > b && b + c > a {
        let s = (a + b + c) / 2.0; // semi-perimeter
        let area = (s * (s - a) * (s - b) * (s - c)).sqrt();
        area.round() // Round to 2 decimal points
    } else {
        -1.0
    }
}

fn main() {
    let a = 3.0;
    let b = 4.0;
    let c = 5.0;

    let area = triangle_area(a, b, c);
    println!("The area of the triangle is: {}", area);
}
```

Note: The above program will not round to exactly 2 decimal points as per the specification due to a minor oversight. The `.round()` function rounds to the nearest whole number, not to a specific number of decimal places. To round to 2 decimal places, you would need to use a different approach, such as the following:

```rust
(area * 100.0).round() / 100.0
```

This will ensure that the result is rounded to two decimal places. Here is the corrected function:

```rust
fn triangle_area(a: f64, b: f64, c: f64) -> f64 {
    if a + b > c && a + c > b && b + c > a {
        let s = (a + b + c) / 2.0; // semi-perimeter
        let area = (s * (s - a) * (s - b) * (s - c)).sqrt();
        (area * 100.0).round() / 100.0 // Round to 2 decimal points
    } else {
        -1.0
    }
}

fn main() {
    let a = 3.0;
    let b = 4.0;
    let c = 5.0;

    let area = triangle_area(a, b, c);
    println!("The area of the triangle is: {:.2}", area);
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
