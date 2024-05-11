
fn prime_fib(n: i32) -> u64 {
    if n < 1 {
        panic!("Input must be a positive integer");
    }
    let mut num_primes = 0; // The count of prime Fibonacci numbers found
    let mut current_fib = (1, 1); // Fibonacci sequence tuple (fib_n-2, fib_n-1)

    // Using u64 here as Fibonacci and prime numbers can grow large,
    // and we're not given an upper limit for n.
    let mut primes: Vec<u64> = vec![2]; // Vector to store prime numbers we've found
    let mut fib_prime: u64; // The current Fibonacci number we're checking if it's prime

    while num_primes < n {
        // Generate the next Fibonacci number
        let next_fib = current_fib.0 + current_fib.1;
        current_fib = (current_fib.1, next_fib);

        // Check if the Fibonacci number is prime
        if is_prime(next_fib, &primes) {
            // Is prime, so increase our count and add it to our primes list
            fib_prime = next_fib;
            num_primes += 1;
            primes.push(fib_prime);
        }
    }

    primes[n as usize - 1]
}

// Helper function to determine if a number is prime, using known primes
fn is_prime(num: u64, primes: &[u64]) -> bool {
    if num < 2 {
        return false;
    }

    // Check divisibility by known primes
    for &prime in primes {
        if num == prime {
            // num is already in our list of primes
            return true;
        } else if num % prime == 0 {
            // num is divisible by a known prime, so it's not a new prime
            return false;
        } else if prime * prime > num {
            // If we've checked all primes less than the square root of num,
            // num is prime (by the mathematical property)
            return true;
        }
    }

    // Check divisibility by odd numbers beyond the known primes
    let mut i = *primes.last().unwrap() + 2; // Start from the next odd number after the last prime found
    while i * i <= num {
        if num % i == 0 {
            return false;
        }
        i += 2;
    }

    true
}

fn main() {
    let n = 10; // Example to get the 10th prime Fibonacci number
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
