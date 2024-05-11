
fn right_angle_triangle(a: f32, b: f32, c: f32) -> bool {
    const EPSILON: f32 = 0.00001; // A small epsilon to account for floating point precision issues
    let (square_a, square_b, square_c) = (a * a, b * b, c * c);

    // Check for each possible right angle
    (square_a + square_b - square_c).abs() < EPSILON ||
    (square_a + square_c - square_b).abs() < EPSILON ||
    (square_b + square_c - square_a).abs() < EPSILON
}

fn main() {
    println!("{}", right_angle_triangle(3.0, 4.0, 5.0)); // true
    println!("{}", right_angle_triangle(5.0, 5.0, 5.0)); // false
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
