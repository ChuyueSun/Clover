
fn fib4(n: i32) -> i64 {
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
            let mut e;
            for _ in 4..=n {
                e = a + b + c + d;
                a = b;
                b = c;
                c = d;
                d = e;
            }
            d
        }
    }
}

fn main() {
    // Test cases
    println!("{}", fib4(0)); // 0
    println!("{}", fib4(1)); // 0
    println!("{}", fib4(2)); // 2
    println!("{}", fib4(3)); // 0
    println!("{}", fib4(4)); // 2
    println!("{}", fib4(5)); // 2
    println!("{}", fib4(6)); // 4
    println!("{}", fib4(7)); // 6
    println!("{}", fib4(8)); // 12
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
