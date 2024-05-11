
/// Determines the output based on the primality of `n`.
///
/// If `n` is a prime number, the function returns the value of `x`.
/// If `n` is not a prime number, the function returns the value of `y`.
///
/// Arguments:
/// * `n`: The number to test for primality.
/// * `x`: The value to return if `n` is prime.
/// * `y`: The value to return if `n` is not prime.
///
/// Returns:
/// - i32: The value of `x` if `n` is prime, or the value of `y` otherwise.
fn x_or_y(n: i32, x: i32, y: i32) -> i32 {
    if is_prime(n) {
        x
    } else {
        y
    }
}

/// Helper function to determine if a number is prime.
///
/// A number is prime if it is greater than 1 and has no divisors other
/// than 1 and itself.
///
/// Arguments:
/// * `number`: The number to test for primality.
///
/// Returns:
/// - bool: `true` if `number` is prime, `false` otherwise.
fn is_prime(number: i32) -> bool {
    if number <= 1 {
        return false;
    }
    if number <= 3 {
        return true;
    }
    if number % 2 == 0 || number % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= number {
        if number % i == 0 || number % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn main() {
    // Example usage:
    let n = 5;
    let x = 10;
    let y = 20;
    println!("Result: {}", x_or_y(n, x, y)); // Should print 10 because 5 is prime
}

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
