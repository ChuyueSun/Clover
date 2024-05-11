
fn prime_fib(n: i32) -> u64 {
    fn is_prime(num: u64) -> bool {
        if num <= 1 {
            return false;
        }
        for i in 2..=((num as f64).sqrt() as u64) {
            if num % i == 0 {
                return false;
            }
        }
        true
    }

    fn fib_number(n: usize) -> u64 {
        let mut a: u64 = 0;
        let mut b: u64 = 1;
        for _ in 0..n {
            let next = a + b;
            a = b;
            b = next;
        }
        a
    }

    let mut count = 0;
    let mut i = 0;
    while count < n {
        let fib = fib_number(i);
        if is_prime(fib) {
            count += 1;
        }
        i += 1;
    }
    fib_number(i - 1)
}

fn main() {
    // Example usage:
    // To get the 1st prime Fibonacci number (i.e., n = 1), uncomment the line below
    // println!("1st prime Fibonacci number is: {}", prime_fib(1));
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
