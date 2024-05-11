
fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }

    if n <= 3 {
        return true;
    }

    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }

    true
}

fn fib(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut a = 0;
            let mut b = 1;
            for _ in 2..=n {
                let next = a + b;
                a = b;
                b = next;
            }
            b
        }
    }
}

fn prime_fib(n: i32) -> u64 {
    let mut count = 0;
    let mut i = 0;
    loop {
        let fib_num = fib(i);
        if is_prime(fib_num) {
            count += 1;
            if count == n as u64 {
                return fib_num;
            }
        }
        i += 1;
    }
}

fn main() {
    // Example usage:
    // This will print the 6th Fibonacci prime number.
    println!("{}", prime_fib(6));
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
