
/// Computes the n-th element of the Fib4 number sequence. This sequence is similar to the Fibonacci sequence, 
/// but is defined with the base cases fib4(0) -> 0, fib4(1) -> 0, fib4(2) -> 2, fib4(3) -> 0 and for n > 3,
/// fib4(n) is the sum of the previous four terms in the sequence.
///
/// # Arguments
///
/// * `n` - The position of the element in the Fib4 sequence to compute.
///
/// # Returns
///
/// An i64 representing the n-th element of the Fib4 number sequence.
///
/// # Examples
///
/// ```
/// let element = fib4(5);
/// assert_eq!(element, 4);
/// ```
fn fib4(n: i32) -> i64 {
    match n {
        0 => 0,
        1 => 0,
        2 => 2,
        3 => 0,
        _ => {
            let mut a = 0;
            let mut b = 0;
            let mut c = 2;
            let mut d = 0;
            let mut temp;

            for _ in 4..=n {
                temp = d;
                d = c + b + a + d;
                a = b;
                b = c;
                c = temp;
            }

            d
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib4() {
        assert!(fib4(5) == 4);
        assert!(fib4(8) == 28);
        assert!(fib4(10) == 104);
        assert!(fib4(12) == 386);
    }

}
