
/// Checks if the given sides of a triangle form a right-angled triangle.
///
/// # Arguments
///
/// * `a`: The length of the first side of the triangle.
/// * `b`: The length of the second side of the triangle.
/// * `c`: The length of the third side of the triangle.
///
/// # Returns
///
/// * `true` if the sides form a right-angled triangle.
/// * `false` otherwise.
///
/// # Examples
///
/// ```
/// assert_eq!(right_angle_triangle(3.0, 4.0, 5.0), true);
/// assert_eq!(right_angle_triangle(1.0, 1.0, 1.0), false);
/// ```
fn right_angle_triangle(a: f32, b: f32, c: f32) -> bool {
    // The sides are sorted to simplify the comparison logic that follows.
    let mut sides = [a, b, c];
    sides.sort_by(|a, b| a.partial_cmp(b).unwrap());

    // Use the Pythagorean theorem (a^2 + b^2 = c^2) to check for a right-angled triangle.
    (sides[0].powi(2) + sides[1].powi(2) - sides[2].powi(2)).abs() < 1e-5
}

fn main() {
    // You can test the function with some examples here if you like.
    println!("Is triangle with sides 3, 4, 5 right-angled? {}", right_angle_triangle(3.0, 4.0, 5.0));
    println!("Is triangle with sides 1, 1, 1 right-angled? {}", right_angle_triangle(1.0, 1.0, 1.0));
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
