
fn fibfib(n: i32) -> i64 {
    match n {
        0 => 0,
        1 => 0,
        2 => 1,
        _ => {
            let mut a = 0;
            let mut b = 0;
            let mut c = 1;
            for _ in 3..=n {
                let next = a + b + c;
                a = b;
                b = c;
                c = next;
            }
            c
        }
    }
}

fn main() {
    // Example usage: compute the 10th element of the fibfib sequence
    let result = fibfib(10);
    println!("The 10th element of the fibfib sequence is: {}", result);
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
