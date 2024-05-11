
fn is_prime(num: u64) -> bool {
    if num <= 1 {
        return false;
    }
    if num <= 3 {
        return true;
    }
    if num % 2 == 0 || num % 3 == 0 {
        return false;
    }
    let mut i: u64 = 5;
    while i * i <= num {
        if num % i == 0 || num % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn fib(n: u64) -> u64 {
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    let mut count: u64 = 0;

    while count < n {
        let temp = a;
        a = b;
        b = temp + b;
        count += 1;
    }

    a
}

fn prime_fib(n: u32) -> u64 {
    let mut count: u32 = 0;
    let mut num: u64 = 0;

    while count < n {
        if is_prime(fib(num)) {
            count += 1;
        }
        num += 1;
    }

    fib(num - 1)
}

fn main() {
    println!("Prime Fibonacci number: {}", prime_fib(6));
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
