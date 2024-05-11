
/// Returns the value of `x` if `n` is a prime number, and the value of `y` otherwise.
/// 
/// # Arguments:
/// 
/// * `n`: The integer to test for primality.
/// * `x`: The value to return if `n` is prime.
/// * `y`: The value to return if `n` is not prime.
fn x_or_y(n: i32, x: i32, y: i32) -> i32 {
    if is_prime(n) {
        x
    } else {
        y
    }
}

/// Helper function that checks if a number is prime.
/// 
/// # Arguments:
/// 
/// * `num`: The number to check.
fn is_prime(num: i32) -> bool {
    if num <= 1 {
        return false;
    }
    for i in 2..((num as f64).sqrt() as i32 + 1) {
        if num % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    // Example usage:
    let n = 5;
    let x = 10;
    let y = 20;
    println!("Result: {}", x_or_y(n, x, y));
}
`
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_x_or_y() {
        assert_eq!(x_or_y(7, 34, 12), 34);
        assert_eq!(x_or_y(15, 8, 5), 5);
        assert_eq!(x_or_y(3, 33, 5212), 33);
        assert_eq!(x_or_y(1259, 3, 52), 3);
        assert_eq!(x_or_y(7919, -1, 12), -1);
        assert_eq!(x_or_y(3609, 1245, 583), 583);
        assert_eq!(x_or_y(91, 56, 129), 129);
        assert_eq!(x_or_y(6, 34, 1234), 1234);
        assert_eq!(x_or_y(1, 2, 0), 0);
        assert_eq!(x_or_y(2, 2, 0), 2);
    }

}
