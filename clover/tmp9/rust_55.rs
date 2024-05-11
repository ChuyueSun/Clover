
/// Return n-th Fibonacci number.
///
/// # Arguments
///
/// * `n`: The n-th number in the Fibonacci sequence to calculate.
///
/// # Returns
///
/// The n-th Fibonacci number.
///
/// # Examples
///
/// ```
/// let result = fib(10);
/// assert_eq!(result, 55); // 10th Fibonacci number is 55
/// ```
fn fib(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}

fn main() {
    // Example usage:
    println!("The 10th Fibonacci number is: {}", fib(10));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib() {
        assert!(fib(10) == 55);
        assert!(fib(1) == 1);
        assert!(fib(8) == 21);
        assert!(fib(11) == 89);
        assert!(fib(12) == 144);
    }

}
