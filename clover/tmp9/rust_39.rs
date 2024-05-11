
use primal::is_prime;

pub fn prime_fib(n: i32) -> u64 {
    let mut primes_count = 0;
    let mut fib_previous: u64 = 1;
    let mut fib_current: u64 = 1;
    
    if n < 1 {
        return 0; // Zeroth element is not defined, return 0
    }
    
    while primes_count < n {
        // Calculate the next Fibonacci number
        let fib_next = fib_previous + fib_current;
        fib_previous = fib_current;
        fib_current = fib_next;

        // Check if the Fibonacci number is a prime
        if is_prime(fib_current as u64) {
            primes_count += 1;
        }
    }

    fib_previous
}

fn main() {
    println!("The prime Fibonacci number is {}", prime_fib(6));
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
