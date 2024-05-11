
fn f(n: i32) -> Vec<i64> {
    let mut results = Vec::new();
    for i in 1..=n {
        if i % 2 == 0 {
            // even index, calculate factorial
            results.push((1..=i).fold(1, |acc, x| acc * x as i64));
        } else {
            // odd index, calculate sum of numbers from 1 to i
            results.push((1..=i).fold(0, |acc, x| acc + x as i64));
        }
    }
    results
}

fn main() {
    // Example usage:
    println!("{:?}", f(5)); // Output: [1, 2, 3, 8, 15]
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
