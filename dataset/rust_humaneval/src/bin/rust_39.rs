fn main() {}

/*

    prime_fib returns n-th number that is a Fibonacci number and it's also prime.

*/

fn prime_fib(n: i32) -> i32 {
    let mut f1 = 1;
    let mut f2 = 2;
    let mut count = 0;
    while count < n {
        f1 = f1 + f2;
        let m = f1;
        f1 = f2;
        f2 = m;
        let mut isprime = true;
        for w in 2..(f1 as f32).sqrt() as i32 + 1 {
            if f1 % w == 0 {
                isprime = false;
                break;
            }
        }
        if isprime {
            count += 1;
        }
        if count == n {
            return f1;
        }
    }
    0
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
