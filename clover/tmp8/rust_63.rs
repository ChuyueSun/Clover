
fn fibfib(n: i32) -> i64 {
    match n {
        0 => 0,
        1 => 0,
        2 => 1,
        _ => {
            let mut a = 0;
            let mut b = 0;
            let mut c = 1;
            let mut result = 0;

            for _ in 3..=n {
                result = a + b + c;
                a = b;
                b = c;
                c = result;
            }

            result
        }
    }
}

fn main() {
    // Example usage: computing the 10th FibFib number
    println!("fibfib(10) = {}", fibfib(10));
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
