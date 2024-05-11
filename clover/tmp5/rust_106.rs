
fn f(n: i32) -> Vec<i64> {
    let mut results: Vec<i64> = Vec::new();

    for i in 1..=n {
        if i % 2 == 0 {
            results.push(factorial(i));
        } else {
            results.push(sum_to(i));
        }
    }

    results
}

fn factorial(n: i32) -> i64 {
    let mut result: i64 = 1;
    for i in 1..=n {
        result *= i as i64;
    }
    result
}

fn sum_to(n: i32) -> i64 {
    (n as i64) * (n as i64 + 1) / 2
}

fn main() {
    // Example usage:
    let sequence = f(10);
    println!("{:?}", sequence);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f() {
        assert!(f(5) == vec![1, 2, 6, 24, 15]);
        assert!(f(7) == vec![1, 2, 6, 24, 15, 720, 28]);
        assert!(f(1) == vec![1]);
        assert!(f(3) == vec![1, 2, 6]);
    }

}
