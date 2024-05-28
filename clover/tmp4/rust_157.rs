
fn right_angle_triangle(a: f32, b: f32, c: f32) -> bool {
    // Since we don't know which side is the hypotenuse, we check
    // all combinations of Pythagorean theorem: a^2 + b^2 = c^2.
    const EPSILON: f32 = 1e-6; // to account for floating point error
    let sides = [a, b, c];
    for i in 0..3 {
        let x = sides[i];
        let y = sides[(i + 1) % 3];
        let z = sides[(i + 2) % 3];
        if (x * x - (y * y + z * z)).abs() < EPSILON {
            return true;
        }
    }
    false
}

fn main() {
    // Example usage:
    println!("{}", right_angle_triangle(3.0, 4.0, 5.0)); // true, it's a right-angled triangle
    println!("{}", right_angle_triangle(5.0, 5.0, 5.0)); // false, not a right-angled triangle
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_right_angle_triangle() {
        assert_eq!(right_angle_triangle(3.0, 4.0, 5.0), true);
        assert_eq!(right_angle_triangle(1.0, 2.0, 3.0), false);
        assert_eq!(right_angle_triangle(10.0, 6.0, 8.0), true);
        assert_eq!(right_angle_triangle(2.0, 2.0, 2.0), false);
        assert_eq!(right_angle_triangle(7.0, 24.0, 25.0), true);
        assert_eq!(right_angle_triangle(10.0, 5.0, 7.0), false);
        assert_eq!(right_angle_triangle(5.0, 12.0, 13.0), true);
        assert_eq!(right_angle_triangle(15.0, 8.0, 17.0), true);
        assert_eq!(right_angle_triangle(48.0, 55.0, 73.0), true);
        assert_eq!(right_angle_triangle(1.0, 1.0, 1.0), false);
        assert_eq!(right_angle_triangle(2.0, 2.0, 10.0), false);
    }

}