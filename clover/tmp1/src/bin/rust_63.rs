
fn fibfib(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 0,
        2 => 1,
        _ => {
            let mut a = 0;
            let mut b = 0;
            let mut c = 1;
            let mut d;
            for _ in 3..=n {
                d = a + b + c;
                a = b;
                b = c;
                c = d;
            }
            c
        }
    }
}

fn main() {
    println!("fibfib(0): {}", fibfib(0)); // Output: 0
    println!("fibfib(1): {}", fibfib(1)); // Output: 0
    println!("fibfib(2): {}", fibfib(2)); // Output: 1
    println!("fibfib(5): {}", fibfib(5)); // Output: 7
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
