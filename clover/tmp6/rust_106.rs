
fn f(n: i32) -> Vec<i64> {
    let mut result = Vec::new();
    for i in 1..=n {
        if i % 2 == 0 {
            result.push(factorial(i as u64) as i64);
        } else {
            result.push(((i * (i + 1)) / 2) as i64); // Sum of numbers from 1 to i (using the arithmetic series sum formula)
        }
    }
    result
}

fn factorial(num: u64) -> u64 {
    (1..=num).product()
}

fn main() {
    let n = 5;
    let result = f(n);
    println!("{:?}", result);
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
