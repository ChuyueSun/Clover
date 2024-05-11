
fn fib(n: i32) -> i64 {
    match n {
        0 => 0,
        1 | 2 => 1,
        _ => {
            let mut a: i64 = 0;
            let mut b: i64 = 1;
            let mut i = 2;
            while i <= n {
                let next = a + b;
                a = b;
                b = next;
                i += 1;
            }
            b
        }
    }
}

fn main() {
    // Example usage
    println!("10th Fibonacci number is: {}", fib(10));
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
