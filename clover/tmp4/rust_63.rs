
fn fibfib(n: i32) -> i64 {
    match n {
        0 => 0,
        1 => 0,
        2 => 1,
        _ => {
            let mut fibs = (0, 0, 1);
            for _ in 3..=n {
                fibs = (fibs.1, fibs.2, fibs.0 + fibs.1 + fibs.2);
            }
            fibs.2
        }
    }
}

fn main() {
    // Test cases
    println!("{}", fibfib(0)); // Should output 0
    println!("{}", fibfib(1)); // Should output 0
    println!("{}", fibfib(2)); // Should output 1
    println!("{}", fibfib(5)); // Should output 4
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
