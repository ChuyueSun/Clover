
/// Returns the value of `x` if `n` is a prime number, and the value of `y` otherwise.
///
/// # Arguments
///
/// * `n` - A 32-bit signed integer to check if it's prime.
/// * `x` - A 32-bit signed integer to be returned if `n` is prime.
/// * `y` - A 32-bit signed integer to be returned if `n` is not prime.
fn x_or_y(n: i32, x: i32, y: i32) -> i32 {
    if is_prime(n) {
        x
    } else {
        y
    }
}

/// Checks whether the given number `n` is prime.
///
/// # Arguments
///
/// * `n` - The number to check for primality.
fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn main() {
    // Example usage:
    let prime_number = 11;
    let non_prime_number = 4;
    let x_value = 100;
    let y_value = 200;

    println!("Result when n is prime: {}", x_or_y(prime_number, x_value, y_value));
    println!("Result when n is not prime: {}", x_or_y(non_prime_number, x_value, y_value));
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
