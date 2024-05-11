
fn fib4(n: i32) -> u64 {
    match n {
        0 => 0,
        1 => 0,
        2 => 2,
        3 => 0,
        _ => {
            let mut memo = [0, 0, 2, 0];
            for _ in 4..=n {
                let next = memo[0] + memo[1] + memo[2] + memo[3];
                memo[0] = memo[1];
                memo[1] = memo[2];
                memo[2] = memo[3];
                memo[3] = next;
            }
            memo[3]
        }
    }
}

fn main() {
    println!("fib4(4) = {}", fib4(4)); // Should output 2
    println!("fib4(5) = {}", fib4(5)); // Should output 2
    println!("fib4(6) = {}", fib4(6)); // Should output 4
    // Add more tests if desired
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
