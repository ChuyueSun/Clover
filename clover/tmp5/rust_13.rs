
/// Calculates the greatest common divisor (GCD) of two integers a and b using
/// the Euclidean algorithm.
///
/// # Arguments
///
/// * `a` - The first integer to calculate the GCD.
/// * `b` - The second integer to calculate the GCD.
///
/// # Returns
///
/// The greatest common divisor of the two integers a and b.
fn greatest_common_divisor(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a.abs() // Ensure the GCD is positive
}

fn main() {
    // Example usage:
    let gcd = greatest_common_divisor(54, 24);
    println!("The greatest common divisor is {}", gcd);
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn test_greatest_common_divisor() {
        assert!(greatest_common_divisor(3, 7) == 1);
        assert!(greatest_common_divisor(10, 15) == 5);
        assert!(greatest_common_divisor(49, 14) == 7);
        assert!(greatest_common_divisor(144, 60) == 12);
    }

}
