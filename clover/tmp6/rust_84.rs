
fn solve(n: i32) -> String {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .fold(0, |acc, x| acc + x)
        .to_string()
}

fn main() {
    // Example usage:
    let result = solve(123);
    println!("The sum of digits in binary: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert!(solve(1000) == "1");
        assert!(solve(150) == "110");
        assert!(solve(147) == "1100");
        assert!(solve(333) == "1001");
        assert!(solve(963) == "10010");
    }

}
