
fn fibfib(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 0,
        2 => 1,
        _ => {
            let mut memo = vec![0, 0, 1];
            for i in 3..=n as usize {
                let next_value = memo[i - 1] + memo[i - 2] + memo[i - 3];
                memo.push(next_value);
            }
            memo[n as usize]
        }
    }
}

fn main() {
    // Example usage:
    let n = 5;
    println!("fibfib({}) = {}", n, fibfib(n));
}

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_fibfib() {
        assert!(fibfib(2) == 1);
        assert!(fibfib(1) == 0);
        assert!(fibfib(5) == 4);
        assert!(fibfib(8) == 24);
        assert!(fibfib(10) == 81);
        assert!(fibfib(12) == 274);
        assert!(fibfib(14) == 927);
    }

}
