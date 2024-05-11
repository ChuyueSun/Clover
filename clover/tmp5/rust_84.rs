
fn solve(n: i32) -> String {
    let sum_of_digits = (0..)
        .map(|i| ((n / 10_i32.pow(i)) % 10))
        .take_while(|&d| d > 0)
        .sum::<i32>();
    format!("{:b}", sum_of_digits)
}

fn main() {
    // Example usage:
    let result = solve(123);
    println!("The sum of digits in binary is: {}", result);
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
