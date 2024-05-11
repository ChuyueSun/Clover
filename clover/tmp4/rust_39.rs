
fn prime_fib(n: usize) -> u128 {
    // A utility function to check if a number is prime
    fn is_prime(num: u128) -> bool {
        if num <= 1 {
            return false;
        }
        if num <= 3 {
            return true;
        }
        if num % 2 == 0 || num % 3 == 0 {
            return false;
        }
        let mut i = 5;
        while i * i <= num {
            if num % i == 0 || num % (i + 2) == 0 {
                return false;
            }
            i += 6;
        }
        true
    }

    // A utility function that returns the next Fibonacci number
    fn next_fib((a, b): (u128, u128)) -> (u128, u128) {
        (b, a + b)
    }

    if n == 0 {
        return 2;
    }

    let mut count = 0;
    let mut fib_pair = (2, 3);

    loop {
        if is_prime(fib_pair.1) {
            count += 1;
            if count == n {
                return fib_pair.1;
            }
        }
        fib_pair = next_fib(fib_pair);
    }
}

fn main() {
    let n = 10; // For example, finding the 10th prime Fibonacci number
    println!("The {}-th prime Fibonacci number is: {}", n, prime_fib(n));
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
