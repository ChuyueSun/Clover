
/// A function that returns the value of `x` if `n` is a prime number
/// and returns the value of `y` otherwise.
///
/// # Parameters
/// - `n`: The integer to be checked if it is a prime.
/// - `x`: The value to be returned if `n` is a prime number.
/// - `y`: The value to be returned if `n` is not a prime number.
///
/// # Returns
/// - Returns `x` if `n` is prime, or `y` otherwise.
fn x_or_y(n: i32, x: i32, y: i32) -> i32 {
    if n <= 1 {
        return y;
    }
    if n <= 3 {
        return x;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return y;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return y;
        }
        i += 6;
    }
    x
}

fn main() {
    // Example usage:
    let n = 5;
    let x = 10;
    let y = 20;
    let result = x_or_y(n, x, y);
    println!("The result is: {}", result);
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
