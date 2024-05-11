
fn main() {
    // Example of using the function
    println!("5th prime Fib number: {}", prime_fib(5));
}

fn prime_fib(n: i32) -> u64 {
    let mut count = 0;
    let mut fib_prev = 0;
    let mut fib_curr = 1;
    let mut num = 1;

    while count < n {
        if is_prime(num) && is_fibonacci(num, &mut fib_prev, &mut fib_curr) {
            count += 1;
        }
        num += 1;
    }

    num - 1
}

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

fn is_fibonacci(num: u64, fib_prev: &mut u64, fib_curr: &mut u64) -> bool {
    while *fib_curr < num {
        let temp = *fib_curr;
        *fib_curr += *fib_prev;
        *fib_prev = temp;
    }
    *fib_curr == num
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
