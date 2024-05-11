
fn main() {
    // Example usage:
    let n = 6;
    println!("The {}-th prime Fibonacci number is {}", n, prime_fib(n));
}

fn prime_fib(n: i32) -> u64 {
    let mut primes = vec![];
    let mut fib_n_minus_2: u64 = 1;
    let mut fib_n_minus_1: u64 = 1;
    let mut fib_n: u64;
    
    while primes.len() < n as usize {
        fib_n = fib_n_minus_2 + fib_n_minus_1;
        fib_n_minus_2 = fib_n_minus_1;
        fib_n_minus_1 = fib_n;

        if is_prime(fib_n) {
            primes.push(fib_n);
        }
    }

    *primes.last().unwrap()
}

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
