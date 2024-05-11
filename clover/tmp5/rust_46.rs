
fn fib4(n: i32) -> i32 {
    let mut memo = [0, 0, 2, 0];
    if n < 4 {
        return memo[n as usize];
    }
    for _ in 4..=n {
        let next = memo.iter().sum();
        memo = [memo[1], memo[2], memo[3], next];
    }
    memo[3]
}

fn main() {
    println!("fib4(5) = {}", fib4(5));
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
