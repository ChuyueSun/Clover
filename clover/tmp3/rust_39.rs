
fn prime_fib(n: i32) -> u64 {
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
        let mut i = 5;
        while i * i <= num {
            if num % i == 0 || num % (i + 2) == 0 {
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

    let mut prime_fib_counter = 0;
    let mut current_fib_index: u64 = 0;
    loop {
        let fibonacci_number = fib(current_fib_index);
        if is_prime(fibonacci_number) {
            prime_fib_counter += 1;
            if prime_fib_counter == n as u64 {
                return fibonacci_number;
            }
        }
        current_fib_index += 1;
    }
}

fn main() {
    // Test the function with the first prime Fibonacci number
    println!("The 1st prime Fibonacci number is: {}", prime_fib(1));
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
