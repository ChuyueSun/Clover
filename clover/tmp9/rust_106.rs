
fn f(n: i32) -> Vec<i64> {
    let mut result = Vec::new();
    for i in 1..=n {
        let value = if i % 2 == 0 {
            // calculate factorial for even i
            (1..=i).fold(1, |acc, x| acc * x as i64)
        } else {
            // calculate sum for odd i
            (1..=i).fold(0, |acc, x| acc + x as i64)
        };
        result.push(value);
    }
    result
}

fn main() {
    // Example usage:
    let n = 5;
    let res = f(n);
    println!("{:?}", res); // Should print the resulting vector according to the specification
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
