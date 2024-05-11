
fn prime_fib(n: i32) -> u64 {
    fn is_prime(num: u64) -> bool {
        if num <= 1 {
            false
        } else if num <= 3 {
            true
        } else if num % 2 == 0 || num % 3 == 0 {
            false
        } else {
            let mut i = 5;
            while i * i <= num {
                if num % i == 0 || num % (i + 2) == 0 {
                    return false;
                }
                i += 6;
            }
            true
        }
    }

    fn fib(n: u64) -> u64 {
        match n {
            0 => 0,
            1 | 2 => 1,
            _ => fib(n - 1) + fib(n - 2),
        }
    }

    let mut count = 0;
    let mut fib_index = 0;
    loop {
        let fib_number = fib(fib_index);
        if is_prime(fib_number) {
            count += 1;
            if count == n as u64 {
                return fib_number;
            }
        }
        fib_index += 1;
    }
}

fn main() {
    // Example usage:
    println!("5th prime Fibonacci number is {}", prime_fib(5));
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn test_prime_fib() {
        assert_eq!(prime_fib(1), 2);
        assert_eq!(prime_fib(2), 3);
        assert_eq!(prime_fib(3), 5);
        assert_eq!(prime_fib(4), 13);
        assert_eq!(prime_fib(5), 89);
        assert_eq!(prime_fib(6), 233);
        assert_eq!(prime_fib(7), 1597);
        assert_eq!(prime_fib(8), 28657);
        assert_eq!(prime_fib(9), 514229);
        assert_eq!(prime_fib(10), 433494437);
    }

}
