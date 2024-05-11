 the Rust program implementing the function:

```rust
pub fn fib(n: i32) -> i64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut a = 0;
            let mut b = 1;
            let mut i = 2;

            while i <= n {
                let c = a + b;
                a = b;
                b = c;
                i += 1;
            }

            b
        }
    }
}

fn main() {
    let n = 10; // Example usage - get the 10th Fibonacci number
    println!("The {}-th Fibonacci number is: {}", n, fib(n));
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
