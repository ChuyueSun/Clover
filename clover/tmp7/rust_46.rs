
fn fib4(n: i32) -> i32 {
    if n < 2 {
        return 0;
    }
    if n == 2 {
        return 2;
    }

    let mut values = (0, 0, 2, 0);
    for _ in 4..=n {
        values = (values.1, values.2, values.3, values.0 + values.1 + values.2 + values.3);
    }
    values.3
}

fn main() {
    let n = 5; // Example usage
    println!("fib4({}) = {}", n, fib4(n));
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
