
fn fib4(n: i32) -> i64 {
    match n {
        0 => 0,
        1 => 0,
        2 => 2,
        3 => 0,
        _ => {
            let mut results = (0, 0, 2, 0);
            for _ in 4..=n {
                results = (
                    results.1,
                    results.2,
                    results.3,
                    results.0 + results.1 + results.2 + results.3,
                );
            }
            results.3
        }
    }
}

fn main() {
    let n = 10; // Change the value of `n` to calculate a different term.
    println!("The {}-th term of the Fib4 sequence is: {}", n, fib4(n));
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
