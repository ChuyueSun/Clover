
fn fib4(n: i32) -> u64 {
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
            for _ in 4..=n {
                let temp_d = d;
                d = c + b + a + d;
                a = b;
                b = c;
                c = temp_d;
            }
            d
        }
    }
}

fn main() {
    // Example usage:
    let n = 10;
    println!("fib4({}) = {}", n, fib4(n));
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
