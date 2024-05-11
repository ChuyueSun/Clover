
fn fib4(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 0,
        2 => 2,
        3 => 0,
        _ => {
            let mut a = 0;
            let mut b = 0;
            let mut c = 2;
            let mut d = 0;
            let mut result = 0;
            for _ in 4..=n {
                result = a + b + c + d;
                a = b;
                b = c;
                c = d;
                d = result;
            }
            result
        }
    }
}

fn main() {
    // Example usage
    println!("fib4(0) = {}", fib4(0)); // Output: 0
    println!("fib4(5) = {}", fib4(5)); // Output: 4
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
