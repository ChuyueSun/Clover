
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
    // Example usage:
    println!("fibfib(5) = {}", fibfib(5)); // Output should be 4, as fibfib(5) = fibfib(4) + fibfib(3) + fibfib(2) = 2 + 1 + 1 = 4
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
